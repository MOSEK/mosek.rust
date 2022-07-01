//! File : portfolio_3_impact
//!
//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! Description :  Implements a basic portfolio optimization model with transaction costs of order x^(3/2).
//!
//! ```
//! Maximize mu'x
//! Subject to
//!    budget : sum(x)+m't = sum(x0)+w
//!    GT     : (gamma,G'x) in Q^{k+1}
//!    MI     : (t_j,1,|x_j-x0_j|) in P^3(2/3,1/3), j = 1..
//!    x >= 0
//! ```
//!
//! Where
//!
//! - m_i is the transaction cost associated with asset i
//! - gamma is the bound on the standard deviation if the portfolio
//! - mu_i is the expected return on asset i
//! - w is the initial wealth held in cash
//! - x0_i is the initial investment in asset i
//! - G'G is the covariance matrix for assets
//!
//! The MI constraint is not convex due tot he |.| term, so we relax it:
//! ```
//!    MI     : (t_j,1,z_j) in P^3(2/3,1/3), j = 1..
//!             z_j >= |x_j-x0_j|
//!             implemented as
//!                z_j >= x_j-x0_j
//!                z_j >= x0_j-x_j
//! ```
extern crate mosek;
extern crate itertools;
use mosek::{Task,Objsense,Streamtype,Solsta,Soltype};
use itertools::{izip,iproduct};


/// Solve portfolio with market impact terms
///
/// # Arguments
///
/// - `n` number of assets
/// - `mu` vector of expected returns
/// - `m` vector of market impact estimates
/// - `GT` factored covariance matrix
/// - `x0` vector if initial investment
/// - `gamma` bound on risk
/// - `w` initial uninvested wealth
#[allow(non_snake_case)]
fn portfolio(n : i32,
             mu : &[f64],
             m  : &[f64],
             GT : &[f64],
             x0  : &[f64],
             gamma : f64,
             w : f64) -> Result<(Vec<f64>,f64),String> {

    let k = (GT.len() / n as usize) as i32;
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    task.append_vars(3*n)?;

    let allvars : Vec<i32> = (0i32..3*n).collect();
    let var_x = &allvars[0..n as usize];
    let var_t = &allvars[n as usize..2*n as usize];
    let var_xt = &allvars[0..2*n as usize];
    let var_z = &allvars[2*n as usize..3*n as usize];

    for (i,j) in var_x.iter().enumerate() {
        task.put_var_bound(*j,mosek::Boundkey::LO, 0.0, 0.0)?;
        task.put_var_name(*j,format!("x[{}]",i).as_str())?; }
    for (i,j) in var_t.iter().enumerate() {
        task.put_var_bound(*j,mosek::Boundkey::FR, 0.0, 0.0)?;
        task.put_var_name(*j,format!("t[{}]",i).as_str())?;
    }
    for (i,j) in var_z.iter().enumerate() {
        task.put_var_bound(*j,mosek::Boundkey::FR, 0.0, 0.0)?;
        task.put_var_name(*j,format!("z[{}]",i).as_str())?;
    }

    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for i in var_x {
        task.put_c_j(*i,mu[*i as usize])?;
    }

    task.append_cons(1)?;
    let con_budget = 0i32;

    // budget
    task.put_con_name(0,"budget")?;
    let wealth = w + x0.iter().sum::<f64>();
    task.put_a_row(con_budget,
                   &var_xt,
                   (0..n).map(|_| 1.0).chain(m.iter().map(|v| *v)).collect::<Vec<f64>>().as_slice())?;
    task.put_con_bound(con_budget,mosek::Boundkey::FX, wealth,wealth)?;

    // |x-x0| <= z
    {
        let acci = task.get_num_acc()?;

        task.append_afes(2*n as i64)?;
        let afes : Vec<i64> = (0..2*n as i64).collect();

        let ones = vec![1.0; n as usize];
        let minusones = vec![-1.0; n as usize];
        task.put_afe_f_entry_list(&afes[0..n as usize],var_x,ones.as_slice())?;
        task.put_afe_f_entry_list(&afes[0..n as usize],var_z,minusones.as_slice())?;

        task.put_afe_f_entry_list(&afes[n as usize..2*n as usize],var_x,ones.as_slice())?;
        task.put_afe_f_entry_list(&afes[n as usize..2*n as usize],var_z,ones.as_slice())?;

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

    // GT
    {
        let acci = task.get_num_acc()?;
        let afei = task.get_num_afe()?;

        task.append_afes(k as i64 + 1)?;
        let dom = task.append_quadratic_cone_domain(k as i64+1)?;
        task.append_acc_seq(dom,
                            afei,
                            vec![0.0; k as usize + 1].as_slice())?;
        task.put_acc_name(acci,"GT")?;
        task.put_afe_g(afei,gamma)?;

        for ((i,j),v) in iproduct!(0..n,0..n).zip(GT).filter(|(_,v)| **v != 0.0) {
            task.put_afe_f_entry(afei + i as i64, j as i32, *v)?;
        }
    }
    // MI
    {
        let mut acci = task.get_num_acc()?;
        let mut afei = task.get_num_afe()?;
        task.append_afes(n as i64 * 3)?;
        let dom = task.append_primal_power_cone_domain(3,&[2.0/3.0, 1.0/3.0])?;

        for (i,tj,zj,x0j) in izip!(0..n,var_t,var_z,x0) {
            task.put_afe_f_entry(afei,*tj,1.0)?;
            task.put_afe_g(afei+1,1.0)?;
            task.put_afe_f_entry(afei+2,*zj,1.0)?;
            task.put_afe_g(afei+2,- *x0j)?;
            task.append_acc(dom,
                            &[afei,afei+1,afei+2],
                            &[0.0, 0.0, 0.0])?;
            task.put_acc_name(acci,format!("MarketImpact[{}]",i).as_str())?;
            afei += 3;
            acci += 1;
        }
    }

    let _ = task.optimize()?;
    task.write_data("pf-impact.ptf")?;
    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(Streamtype::MSG)?;

    if ! task.solution_def(Soltype::ITR)? {
        return Err("No solultion defined".to_string());
    }
    let solsta = task.get_sol_sta(Soltype::ITR)?;
    if solsta != Solsta::OPTIMAL {
        return Err("Unexpected solution status".to_string());
    }

    let mut level = vec![0.0;n as usize];
    task.get_xx_slice(Soltype::ITR,0,n,level.as_mut_slice())?;
    let obj = task.get_primal_obj(Soltype::ITR)?;
    
    Ok((level,obj))
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n    = 8i32;
    let w    = 1.0;
    let mu   = &[0.07197, 0.15518, 0.17535, 0.08981, 0.42896, 0.39292, 0.32171, 0.18379];
    let x0   = &[0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let GT   = &[0.30758, 0.12146, 0.11341, 0.11327, 0.17625, 0.11973, 0.10435, 0.10638,
                 0.     , 0.25042, 0.09946, 0.09164, 0.06692, 0.08706, 0.09173, 0.08506,
                 0.     , 0.     , 0.19914, 0.05867, 0.06453, 0.07367, 0.06468, 0.01914,
                 0.     , 0.     , 0.     , 0.20876, 0.04933, 0.03651, 0.09381, 0.07742,
                 0.     , 0.     , 0.     , 0.     , 0.36096, 0.12574, 0.10157, 0.0571 ,
                 0.     , 0.     , 0.     , 0.     , 0.     , 0.21552, 0.05663, 0.06187,
                 0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.22514, 0.03327,
                 0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.     , 0.2202 ];
    let gamma = 0.36;
    let m    = &[0.01   , 0.01   , 0.01   , 0.01   , 0.01   , 0.01   , 0.01   , 0.01];

    // let n     = 3i32;
    // let w     = 1.0;
    // let m     = vec![0.01, 0.01, 0.01];
    // let x0    = vec![0.0, 0.0, 0.0];
    // let gamma = 0.05;
    // let mu    = vec![0.1073,  0.0737,  0.0627];
    // let GT    = vec![0.1667,  0.0232,  0.0013,
    //                  0.0000,  0.1033, -0.0022,
    //                  0.0000,  0.0000,  0.0338];

    let (level,obj) = portfolio(n, mu, m, GT, x0, gamma, w)?;

    println!("Solution x = {:?}",level);
    println!("Objective value x = {:?}",obj);

    
    Ok(())
}
