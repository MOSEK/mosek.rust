//! File : portfolio_2_basic.rs
//!
//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! Description :  Implements a basic portfolio optimization model.
//!                Determines points on the efficient frontier.

extern crate mosek;
extern crate itertools;
use mosek::{Task,Objsense,Solsta,Soltype};
use itertools::{iproduct};

/// Solve basic Markowitz portfolio problem for different risk bounds
/// to produce a list of points on the efficient frontier.
///
/// ```
/// Maximize mu'x - alpha * s
/// Subject to
///    budget : sum(x) = sum(x0)+w
///    risk:    s >= || G'x ||
///    x >= 0
/// ```
///
/// # Arguments
///
/// - `n` number of assets
/// - `alphas` list of risk bound (bound on the standard deviation)
/// - `mu` vector of expected returns
/// - `GT` Covariance matrix factor
/// - `x0` vector if initial investment
/// - `w` initial uninvested wealth
#[allow(non_snake_case)]
fn portfolio(n : i32,
             mu : &[f64],
             GT : &[f64],
             x0  : &[f64],
             alphas : &[f64],
             w : f64) -> Result<Vec<(f64,f64)>,String> {
    let k = (GT.len() / n as usize) as i32;
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(t) => t,
        None => return Err("Failed to create task".to_string()),
    };
    //task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    task.append_vars(n+1)?;
    task.append_cons(1)?;

    /* Objective */
    task.put_obj_sense(Objsense::MAXIMIZE)?;

    let x : Vec<i32> = (0i32..n).collect();
    let s = n;

    /* Total budget */
    let total_budget = w + x0.iter().sum::<f64>();

    /* Total budget constraint - set bounds l^c = u^c */
    task.put_con_bound(0i32, mosek::Boundkey::FX, total_budget, total_budget)?;
    task.put_con_name(0i32,"budget")?;
    task.put_c_slice(0,n,mu)?;

    /* x variables. */
    for (j,xj) in x.iter().enumerate() {
        /* Coefficients in the first row of A */
        task.put_aij(0, *xj, 1.0)?;
        /* No short-selling - x^l = 0, x^u = inf */
        task.put_var_bound(*xj, mosek::Boundkey::LO, 0.0, 0.0)?;
        task.put_var_name(*xj, format!("x[{}]",j+1).as_str())?;
    }
    task.put_var_name(s, "s")?;
    task.put_var_bound(s, mosek::Boundkey::FR, 0.0, 0.0)?;

    // risk bound
    // (s,0.5,GT * x) in Q_r
    {
        let acci = task.get_num_acc()?;
        let afei = task.get_num_afe()?;

        task.append_afes(k as i64 + 2)?;
        let dom = task.append_r_quadratic_cone_domain(k as i64+2)?;
        task.append_acc_seq(dom,
                            afei,
                            vec![0.0; k as usize + 2].as_slice())?;
        task.put_acc_name(acci,"variance")?;
        task.put_afe_f_entry(afei,s,1.0)?;
        task.put_afe_g(afei+1,0.5)?;

        for ((i,j),v) in iproduct!(0..n,0..n).zip(GT).filter(|(_,v)| **v != 0.0) {
            task.put_afe_f_entry(afei + i as i64 + 2, j as i32, *v)?;
        }
    }

    let frontier : Vec<(f64,f64)> = alphas.iter().enumerate().filter_map(|(i,alpha)| {
        /* Sets the objective function coefficient for s. */
        if      let Err(_) = task.put_c_j(s, - *alpha) { None }
        else if let Err(_) = task.optimize() { None }
        else if let Ok(solsta) = task.get_sol_sta(Soltype::ITR) {
            match solsta {
                Solsta::OPTIMAL => {
                    let mut xx = vec![0.0; n as usize+1];
                    if let Err(_) = task.get_xx(Soltype::ITR,xx.as_mut_slice()) { None }
                    else {
                        Some((*alpha,mu.iter().zip(xx.iter()).map(|(m,x)| m * x).sum::<f64>()))
                    }
                }
                _ => None
            }
        }
        else {
            None
        }
    }).collect();

    Ok(frontier)
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n : i32 = 3;
    let mu = vec![0.1073,  0.0737,  0.0627];
    let x0 = vec![0.0, 0.0, 0.0];
    let w = 1.0;
    let alphas = vec![0.0, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];
    let GT = vec![0.1667,  0.0232,  0.0013,
                  0.0000,  0.1033, -0.0022,
                  0.0000,  0.0000,  0.0338];

    println!("{:10}   {:10}","alpha","exp.ret.");
    for (alpha,expret) in portfolio(n,mu.as_slice(),GT.as_slice(),x0.as_slice(),alphas.as_slice(),w)? {
        println!("{:10.3e} : {:10.e}",alpha,expret);
    }

    Ok(())
}
