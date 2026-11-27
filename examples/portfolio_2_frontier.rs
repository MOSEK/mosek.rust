//!
//! File : portfolio_2_frontier.rs
//!
//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! Description :  Implements a basic portfolio optimization model.
//!                Determines points on the efficient frontier.
//!

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
        task.put_acc_name(acci,"risk")?;
        task.put_afe_f_entry(afei,s,1.0)?;
        task.put_afe_g(afei+1,0.5)?;

        for ((i,j),v) in iproduct!(0..n,0..n).zip(GT).filter(|(_,v)| **v != 0.0) {
            task.put_afe_f_entry(afei + i as i64 + 2, j as i32, *v)?;
        }
    }

    let frontier : Vec<(f64,f64)> = alphas.iter().filter_map(|alpha| {
        /* Sets the objective function coefficient for s. */
        if      let Err(_) = task.put_c_j(s, - *alpha) { None }
        else if let Err(_) = task.optimize() { None }
        else if let Err(_) = task.write_data(format!("portfolio_2_frontier-{}.ptf",alpha).as_str()) { None }
        else if let Ok(solsta) = task.get_sol_sta(Soltype::ITR) {
            // See https://docs.mosek.com/latest/rustapi/accessing-solution.html about handling solution statuses.
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
    let n : i32 = 8;
    let w  = 1.0;
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
    let alphas = &[0.0, 0.01, 0.1, 0.25, 0.30, 0.35, 0.4, 0.45, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 10.0];

    println!("{:10}   {:10}","alpha","exp.ret.");
    for (alpha,expret) in portfolio(n,
                                    mu,
                                    GT,
                                    x0,
                                    alphas,
                                    w)? {
        println!("{:10.3e} : {:10.e}",alpha,expret);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
