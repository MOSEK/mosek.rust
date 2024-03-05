//!  File : portfolio_4_transcost.rs
//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  Description :  Implements a basic portfolio optimization model
//!                 with fixed setup costs and transaction costs
//!                 as a mixed-integer problem.
//!

extern crate mosek;
use mosek::{Task,Objsense,Streamtype,Soltype,Variabletype,Boundkey,Solsta};
extern crate itertools;
use itertools::{iproduct};

const INF : f64 = 0.0;

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
    }.with_callbacks();

    let k = (GT.len() / n as usize) as i32;
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;


    /* Compute total wealth */
    let w0 = w + x0.iter().sum::<f64>();

    task.append_cons(1i32)?;
    task.append_vars(3*n)?;

    for i in 0i32..n {
        task.put_var_name(i,    format!("x[{}]",i+1).as_str())?;
        task.put_var_name(i+n,  format!("y[{}]",i+1).as_str())?;
        task.put_var_name(i+2*n,format!("z[{}]",i+1).as_str())?;
        task.put_var_type(i+n, Variabletype::TYPE_INT)?;
    }

    let all_vars : Vec<i32> = (0i32..3*n).collect();
    let x = &all_vars[0..n as usize];
    let y = &all_vars[n as usize..2*n as usize];
    let z = &all_vars[2*n as usize..3*n as usize];

    task.put_var_bound_slice_const(0i32,n,  mosek::Boundkey::LO, 0.0,0.0)?;
    task.put_var_bound_slice_const(n,2*n,   mosek::Boundkey::RA, 0.0,1.0)?;
    task.put_var_bound_slice_const(2*n,3*n, mosek::Boundkey::FR, 0.0,0.0)?;

    /* Constraints. */
    task.put_con_name(0,"budget")?;
    {
        let zeros = vec![0i32; n as usize];
        let fones = vec![1.0; n as usize];
        task.put_aij_list(zeros.as_slice(), x, fones.as_slice())?;
        task.put_aij_list(zeros.as_slice(), y, f)?;
        task.put_aij_list(zeros.as_slice(), z, g)?;
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
        let coni = task.get_num_con()?;
        task.append_cons(2 * n)?;
        for i in 0..n {
            task.put_con_name(coni+i,   format!("zabs1[{}]",1 + i).as_str())?;
            task.put_con_name(coni+n+i, format!("zabs2[{}]",1 + i).as_str())?;
        }
        let ones      = vec![1.0; n as usize];
        let minusones = vec![-1.0; n as usize];
        let con_abs1 : Vec<i32> = (coni..coni+n).collect();
        let con_abs2 : Vec<i32> = (coni+n..coni+2*n).collect();
        task.put_aij_list(con_abs1.as_slice(), x, minusones.as_slice())?;
        task.put_aij_list(con_abs1.as_slice(), z, ones.as_slice())?;
        task.put_con_bound_slice(coni,coni+n, vec![Boundkey::LO; n as usize].as_slice(), x0.iter().map(|&v| -v).collect::<Vec<f64>>().as_slice(), vec![INF; n as usize].as_slice())?;
        task.put_aij_list(con_abs2.as_slice(), x, ones.as_slice())?;
        task.put_aij_list(con_abs2.as_slice(), z, ones.as_slice())?;
        task.put_con_bound_slice(coni+n,coni+n*2, vec![Boundkey::LO; n as usize].as_slice(), x0, vec![INF; n as usize].as_slice())?;
    }

    // Switch
    {
        let coni = task.get_num_con()?;
        task.append_cons(n)?;
        for i in 0..n {
            task.put_con_name(coni + i, format!("switch[{}]",i+1).as_str())?;
        }

        let conlist : Vec<i32> = (coni..coni+n).collect();
        task.put_aij_list(conlist.as_slice(), z, vec![1.0; n as usize].as_slice())?;
        task.put_aij_list(conlist.as_slice(), y, vec![-w0; n as usize].as_slice())?;

        task.put_con_bound_slice_const(coni,coni+n, Boundkey::UP, 0.0,0.0)?;
    }

    let _ = task.optimize()?;
    task.write_data("portfolio_4_transcost.ptf")?;

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(Streamtype::MSG)?;

    // Check if the integer solution is an optimal point
    if task.get_sol_sta(Soltype::ITG)? != Solsta::INTEGER_OPTIMAL {
        // See https://docs.mosek.com/latest/rustapi/accessing-solution.html about handling solution statuses.
        eprintln!("Solution not optimal!");
        std::process::exit(1);
    }

    let mut xx = vec![0.0;n as usize];
    task.get_xx_slice(Soltype::ITG, 0,n, xx.as_mut_slice())?;
    let expret = xx[0..n as usize].iter().zip(mu.iter()).map(|(a,b)| a*b).sum::<f64>();

    Ok((xx,expret))
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n = 8i32;
    let w = 1.0;
    let mu = &[0.07197, 0.15518, 0.17535, 0.08981, 0.42896, 0.39292, 0.32171, 0.18379];
    let x0 = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let GT = &[ 0.30758, 0.12146, 0.11341, 0.11327, 0.17625, 0.11973, 0.10435, 0.10638,
                0.     , 0.25042, 0.09946, 0.09164, 0.06692, 0.08706, 0.09173, 0.08506,
                0.     , 0.     , 0.19914, 0.05867, 0.06453, 0.07367, 0.06468, 0.01914,
                0.     , 0.     , 0.     , 0.20876, 0.04933, 0.03651, 0.09381, 0.07742,
                0.     , 0.     , 0.     , 0.     , 0.36096, 0.12574, 0.10157, 0.0571 ,
                0.     , 0.     , 0.     , 0.     , 0.     , 0.21552, 0.05663, 0.06187,
                0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.22514, 0.03327,
                0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.2202 ];
    let f = vec![0.01; n as usize];
    let g = vec![0.001; n as usize];
    let gamma = 0.36;

    let (level,expret) = portfolio(n,
                                   mu,
                                   f.as_slice(),
                                   g.as_slice(),
                                   GT,
                                    x0,
                                   gamma,
                                   w)?;

    println!("Expected return {:.4e} for gamma {:.4e}\n", expret, gamma);
    println!("Solution vector = {:?}\n", level);
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
