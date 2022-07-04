//!  File : portfolio_4_transcost
//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  Description :  Implements a basic portfolio optimization model
//!                 with fixed setup costs and transaction costs
//!                 as a mixed-integer problem.
//!

extern crate mosek;
use mosek::{Task,Objsense,Streamtype,Soltype};
extern crate itertools;
use itertools::{izip,iproduct};


/// Optimize expected return on an investment with transaction cost.
///
/// ```
/// Maximize mu'x
/// Such That
///   budget: sum(x) + f'y + g'z = w0+sum(x0)
///   risk:   gamma > || G'x ||
///   ACC:    z_j > |x0_j-x_j|
///   DJC:    [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
///           // z_j < U_j y_j, y in {0,1}
///           y free
///           x > 0
/// Where f_i is the fixed cost of a transaction in asset i,
///       g_i is the cost per unit of a transaction in asset i
/// ```
/// # Arguments
///
/// - `n` number of assets
/// - `mu` vector of expected returns
/// - `f` vector of fixed transaction costs
/// - 'g' vector of continuous proportional transaction costs
/// - `GT` Covariance matrix factor
/// - `x0` vector if initial investment
/// - `gamma` risk bound (bound on the standard deviation)
/// - `w` initial uninvested wealth
#[allow(non_snake_case)]
fn portfolio(n : i32,
             mu : &[f64],
             f  : &[f64],
             g  : &[f64],
             GT : &[f64],
             x0  : &[f64],
             gamma : f64,
             w : f64) -> Result<(),String> {
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };

    let k = (GT.len() / n as usize) as i32;
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;


    /* Compute total wealth */
    let w0 = w + x0.iter().sum::<f64>();

    task.append_cons(1i32)?;
    task.append_vars(3*n)?;

    for i in 0i32..n {
        task.put_var_name(i,    format!("x[{}]",i).as_str())?;
        task.put_var_name(i+n,  format!("y[{}]",i).as_str())?;
        task.put_var_name(i+2*n,format!("z[{}]",i).as_str())?;
    }

    task.put_var_bound_slice(0i32,3*n,
                             vec![mosek::Boundkey::FR; 3*n as usize].as_slice(),
                             vec![0.0; 3*n as usize].as_slice(),
                             vec![0.0; 3*n as usize].as_slice())?;

    let x_base = 0i32;
    let y_base = n;
    let z_base = 2*n;

    /* Constraints. */
    task.put_con_name(0,"budget")?;
    task.put_a_row(0i32,
                   (0i32..3*n).collect::<Vec<i32>>().as_slice(),
                   (0..n).map(|_| 1.0).chain(f.iter().map(|v| *v)).chain(g.iter().map(|v| *v)).collect::<Vec<f64>>().as_slice())?;
    task.put_con_bound(0i32,mosek::Boundkey::FX,w0,w0)?;

    // objective
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for (i,mui) in (0..n).zip(mu.iter()) {
        task.put_c_j(x_base+i, *mui)?;
    }

    // risk bound
    {
        let acci = task.get_num_acc()?;
        let afei = task.get_num_afe()?;

        task.append_afes(k as i64 + 1)?;
        let dom = task.append_quadratic_cone_domain(k as i64+1)?;
        task.append_acc_seq(dom,
                            afei,
                            vec![0.0; k as usize + 1].as_slice())?;
        task.put_acc_name(acci,"risk")?;
        task.put_afe_g(afei,gamma)?;

        for ((i,j),v) in iproduct!(0..n,0..n).zip(GT).filter(|(_,v)| **v != 0.0) {
            task.put_afe_f_entry(afei + i as i64 + 1, j as i32, *v)?;
        }
    }

    // z_i > |x_i-x0_i|
    {
        let afei = task.get_num_afe()?;
        let dom = task.append_rplus_domain(n as i64)?;
        // z_i > x_i - x0_i <=> z_i - x_i + x0_i > 0
        task.append_afes(n as i64)?;
        task.put_afe_f_entry_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                                  (x_base..x_base+n).chain(z_base..z_base+n).collect::<Vec<i32>>().as_slice(),
                                  (0..n).map(|_| -1.0 ).chain((0..n).map(|_| 1.0)).collect::<Vec<f64>>().as_slice())?;
        task.put_afe_g_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                            x0)?;
        task.append_acc(dom,
                        (afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                        (0..n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
        let afei = task.get_num_afe()?;
        // z_i > x0_i - x_i <=> z_i + z_i - x0_i > 0
        task.append_afes(n as i64)?;
        task.put_afe_f_entry_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                                  (x_base..x_base+n).chain(z_base..z_base+n).collect::<Vec<i32>>().as_slice(),
                                  (0..2*n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
        task.put_afe_g_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                            x0.iter().map(|v| -v).collect::<Vec<f64>>().as_slice())?;
        task.append_acc(dom,
                        (afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                        (0..n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
    }

    //DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
    {
        task.append_djcs(n as i64)?;
        let domeq = task.append_rzero_domain(1)?;
        for i in 0..n {
            let afei = task.get_num_afe()?;
            task.append_afes(3)?;
            // y_j = 0
            task.put_afe_f_entry(afei,y_base+i,1.0)?;
            // x_j = x0_j
            task.put_afe_f_entry(afei+1,x_base+i,1.0)?;
            // y_j = 1
            task.put_afe_f_entry(afei+2,y_base+i,1.0)?;
            task.put_djc(i as i64,
                         &[domeq,domeq,domeq],
                         &[afei,afei+1,afei+2],
                         &[0.0, x0[i as usize], 1.0],
                         &[2,1])?;
        }
    }


    let _ = task.optimize()?;

    /* Dump the problem to a human readable OPF file. */
    task.write_data("portfolio_4_transcost.ptf")?;

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(Streamtype::MSG)?;

    let mut xx = vec![0.0;(3*n) as usize];
    task.get_xx(Soltype::ITG, xx.as_mut_slice())?;
    let expret = xx[0..n as usize].iter().zip(mu.iter()).map(|(a,b)| a*b).sum::<f64>();


    println!("\nExpected return {:.4e} for gamma {:.4e}\n", expret, gamma);
    Ok(())
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n       = 3i32;
    let w       = 1.0;
    let x0      = vec![0.0, 0.0, 0.0];
    let gamma   = 0.05;
    let mu      = vec![0.1073,  0.0737,  0.0627];
    let f       = vec![0.01, 0.01, 0.01];
    let g       = vec![0.001, 0.001, 0.001];
    let GT      = vec![0.1667,  0.0232,  0.0013,
                       0.0000,  0.1033, -0.0022,
                       0.0000,  0.0000,  0.0338 ];
    portfolio(n,
              mu.as_slice(),
              f.as_slice(),
              g.as_slice(),
              GT.as_slice(),
              x0.as_slice(),
              gamma,
              w)?;

    Ok(())
}
