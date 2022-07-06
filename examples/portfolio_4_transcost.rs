//!  File : portfolio_4_transcost
//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  Description :  Implements a basic portfolio optimization model
//!                 with fixed setup costs and transaction costs
//!                 as a mixed-integer problem.
//!

#[path = "common.rs"]
mod common;

extern crate mosek;
use mosek::{Task,Objsense,Streamtype,Soltype,Variabletype};
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
             w : f64) -> Result<(Vec<f64>,f64),String> {
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
        task.put_var_type(i+n, Variabletype::TYPE_INT);
    }

    task.put_var_bound_slice_const(0i32,n,  mosek::Boundkey::LO, 0.0,0.0)?;
    task.put_var_bound_slice_const(n,2*n,   mosek::Boundkey::FR, 0.0,0.0)?;
    task.put_var_bound_slice_const(2*n,3*n, mosek::Boundkey::RA, 0.0,1.0)?;
    let all_vars : Vec<i32> = (0i32..3*n).collect();
    let x = &all_vars[0..n as usize];
    let y = &all_vars[n as usize..2*n as usize];
    let z = &all_vars[2*n as usize..3*n as usize];

    /* Constraints. */
    task.put_con_name(0,"budget")?;
    {
        let ones = vec![0i32; n as usize];
        let fones = vec![0.0; n as usize];
        task.put_aij_list(ones.as_slice(), x, fones.as_slice());
        task.put_aij_list(ones.as_slice(), y, f);
        task.put_aij_list(ones.as_slice(), z, g);
        task.put_con_bound(0i32,mosek::Boundkey::FX,w0,w0)?;
    }

    // objective
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for (xi,mui) in x.iter().zip(mu.iter()) {
        task.put_c_j(*xi, *mui)?;
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

    // |x-x0| <= z
    {
        let acci = task.get_num_acc()?;

        task.append_afes(2*n as i64)?;
        let afes : Vec<i64> = (0..2*n as i64).collect();

        let ones = vec![1.0; n as usize];
        let minusones = vec![-1.0; n as usize];
        task.put_afe_f_entry_list(&afes[0..n as usize],x,ones.as_slice())?;
        task.put_afe_f_entry_list(&afes[0..n as usize],z,minusones.as_slice())?;

        task.put_afe_f_entry_list(&afes[n as usize..2*n as usize],x,ones.as_slice())?;
        task.put_afe_f_entry_list(&afes[n as usize..2*n as usize],z,ones.as_slice())?;

        let domneg = task.append_rminus_domain(n as i64)?;
        task.append_acc(domneg,
                        &afes[0..n as usize],
                        x0)?;
        let dompos = task.append_rplus_domain(n as i64)?;
        task.append_acc(dompos,
                        &afes[n as usize..2*n as usize],
                        x0)?;
        task.put_acc_name(acci,"(x-z)<x0")?;
        task.put_acc_name(acci+1,"(x+z)>x0")?;
    }


    // Switch
    {
        let coni = task.get_num_con()?;
        task.append_cons(n)?;
        for i in 0..n {
            task.put_con_name(coni + i, format!("switch[{}]",i))?;
        }

        let conlist : Vec<i32> = (coni..coni+n).collect();
        task.put_aij_list(conlist.as_slice(), z, vec![1.0; n as usize].as_slice())?;
        task.put_aij_list(conlist.as_slice(), y, vec![-w0; n as usize].as_slice())?;

        tak.put_con_bound_slice_const(conlist.as_slice(), Boundkey::UP, 0.0,0.0);
        }
    }

    let _ = task.optimize()?;

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(Streamtype::MSG)?;

    let mut xx = vec![0.0;(3*n) as usize];
    task.get_xx(Soltype::ITG, xx.as_mut_slice())?;
    let expret = xx[0..n as usize].iter().zip(mu.iter()).map(|(a,b)| a*b).sum::<f64>();

    Ok((xx,expret))
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let (_level,expret) = portfolio(common::Data1::n,
                                    common::Data1::mu,
                                    common::Data1::fixed_tcost,
                                    common::Data1::prop_tcost,
                                    common::Data1::GT,
                                    common::Data1::x0,
                                    common::Data1::gamma,
                                    common::Data1::w)?;

    println!("\nExpected return {:.4e} for gamma {:.4e}\n", expret, common::Data1::gamma);
    Ok(())
}
