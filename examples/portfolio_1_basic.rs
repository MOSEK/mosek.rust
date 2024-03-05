//!
//! File : portfolio_1_basic.rs
//!
//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! Description :  Implements a basic portfolio optimization model.
//!
//! More details can be found at <https://docs.mosek.com/latest/capi/case-portfolio.html#doc-optimizer-case-portfolio>
//!

extern crate mosek;
extern crate itertools;
use mosek::{Task,Objsense,Streamtype,Soltype,Solsta};
use itertools::{iproduct};

/// Solve basic Markowitz portfolio problem: Maximize expected return
/// while bounded the estimated risk.
///
/// ```
/// Maximize mu'x
/// Subject to
///    budget : sum(x) = sum(x0)+w
///    risk:    gamma^2 >= || G'x ||^2
///    x >= 0
/// ```
///
/// # Arguments
///
/// - `n` number of assets
/// - `gamma` risk bound (bound on the standard deviation)
/// - `mu` vector of expected returns
/// - `GT` Covariance matrix factor
/// - `x0` vector if initial investment
/// - `w` initial uninvested wealth
#[allow(non_snake_case)]
fn portfolio(n     : i32,     // number of assets
             gamma : f64, // risk bound: maximum stddev
             mu    : &[f64], // vector of expected returns
             GT    : &[f64], // covariance matrix factor
             x0    : &[f64], // initial investment
             w     : f64)     // initial wealth
             -> Result<(Vec<f64>,f64),String> {

    let k = (GT.len() / n as usize) as i32;
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    }.with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Total budget */
    let total_budget = w + x0.iter().sum::<f64>();

    /* Constraints. */
    task.append_cons(1i32)?;
    /* Variables. */
    task.append_vars(n)?;

    let x : Vec<i32> = (0i32..n).collect();

    /* Total budget constraint - set bounds l^c = u^c */
    task.put_con_bound(0i32, mosek::Boundkey::FX, total_budget, total_budget)?;
    task.put_con_name(0i32,"budget")?;

    task.put_obj_sense(Objsense::MAXIMIZE)?;
    task.put_c_slice(0,n,mu)?;

    /* x variables. */
    for (j,xj) in x.iter().enumerate() {
        /* Coefficients in the first row of A */
        task.put_aij(0, *xj, 1.0)?;
        /* No short-selling - x^l = 0, x^u = inf */
        task.put_var_bound(*xj, mosek::Boundkey::LO, 0.0, 0.0)?;
        task.put_var_name(*xj, format!("x[{}]",j+1).as_str())?;
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

    /* Dump the problem to a human readable OPF file. */
    // task.write_data("portfolio_1_basic.ptf")?;

    let _trm = task.optimize()?;

    // Check if the interior point solution is an optimal point
    if task.get_sol_sta(Soltype::ITR)? != Solsta::OPTIMAL {
        // See https://docs.mosek.com/latest/rustapi/accessing-solution.html about handling solution statuses.
        eprintln!("Solution not optimal!");
        std::process::exit(1);
    }

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(Streamtype::MSG)?;

    task.write_data("portfolio_1_basic.ptf")?;

    /* Read the x variables one by one and compute expected return. */
    /* Can also be obtained as value of the objective. */
    let mut level = vec![0.0;n as usize];
    task.get_xx_slice(Soltype::ITR,0,n,level.as_mut_slice())?;

    let expret = task.get_primal_obj(Soltype::ITR)?;

    Ok((level.to_vec(),expret))
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n    = 8i32;
    let w    = 59.0;
    let mu   = &[0.07197349, 0.15518171, 0.17535435, 0.0898094 , 0.42895777, 0.39291844, 0.32170722, 0.18378628];
    let x0   = &[8.0, 5.0, 3.0, 5.0, 2.0, 9.0, 3.0, 6.0];
    let gamma  = 36.0;
    let GT     = &[ 0.30758, 0.12146, 0.11341, 0.11327, 0.17625, 0.11973, 0.10435, 0.10638,
                    0.     , 0.25042, 0.09946, 0.09164, 0.06692, 0.08706, 0.09173, 0.08506,
                    0.     , 0.     , 0.19914, 0.05867, 0.06453, 0.07367, 0.06468, 0.01914,
                    0.     , 0.     , 0.     , 0.20876, 0.04933, 0.03651, 0.09381, 0.07742,
                    0.     , 0.     , 0.     , 0.     , 0.36096, 0.12574, 0.10157, 0.0571 ,
                    0.     , 0.     , 0.     , 0.     , 0.     , 0.21552, 0.05663, 0.06187,
                    0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.22514, 0.03327,
                    0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.2202 ];

    let (level,expret) = portfolio(n,gamma,mu,GT,x0,w)?;

    println!("Solution x = {:?}",level);
    println!("Expected return {:.4e} for gamma = {:.4e}", expret, gamma);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
