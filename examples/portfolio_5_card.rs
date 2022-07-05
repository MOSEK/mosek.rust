//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  Description :  Implements a basic portfolio optimization model
//!                 with cardinality constraints on number of assets traded.
//!

#[path = "common.rs"]
mod common;

extern crate mosek;
use mosek::{Task,Objsense,Soltype};
extern crate itertools;
use itertools::{izip,iproduct};


/// Solve the Markowitz portfolio problem with cardinality constraints.
///
/// ```
/// Maximize mu'x
/// Such That
///   budget:      𝐞'x = w0+sum(x0)
///   risk:        γ^2 > ||G'x||^2
///   cardinality: 𝐞'y < p
///                [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
///                y ∈ R^n
///                y ∈ R+^n
/// ```
///
/// Where
///
/// - `y_j` ∈ `{0,1}` is an indicator of whether we change the
///   investment in asset j, that is if `y_j=0`, then we do not change
///   investment in assert `j`.
///
/// # Arguments
///
/// - `n` number of assets
/// - `mu` vector of expected returns
/// - `GT` Covariance matrix factor
/// - `x0` vector if initial investment
/// - `gamma` risk bound (bound on the standard deviation)
/// - `w` initial uninvested wealth
/// - `p` maximum number of assets to invest in
#[allow(non_snake_case)]
fn portfolio(n     : i32,
             mu    : &[f64],
             GT    : &[f64],
             x0    : &[f64],
             gamma : f64,
             p     : i32,
             w     : f64) -> Result<(Vec<f64>,f64),String> {

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };

    let k = (GT.len() / n as usize) as i32;
    // task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Compute total wealth */
    let w0 = w + x0.iter().sum::<f64>();

    task.append_cons(2i32)?;
    task.append_vars(2*n)?;

    let all_vars : Vec<i32> = (0..2*n).collect();
    let x = &all_vars[0..n as usize];
    let y = &all_vars[n as usize..2*n as usize];

    task.put_var_bound_slice_const(0,n,mosek::Boundkey::LO,0.0,0.0)?;
    task.put_var_bound_slice_const(n,2*n,mosek::Boundkey::RA,0.0,1.0)?;

    for (i,j) in x.iter().enumerate() {
        task.put_var_name(*j,format!("x[{}]",i).as_str())?;
    }
    for (i,j) in y.iter().enumerate() {
        task.put_var_name(*j,format!("y[{}]",i).as_str())?;
    }

    // objective
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for (j,mui) in x.iter().zip(mu.iter()) {
        task.put_c_j(*j, *mui)?;
    }


    let n_ones = vec![1.0; n as usize];
    // budget constraint
    task.put_con_name(0,"budget")?;
    task.put_a_row(0,
                   x,
                   n_ones.as_slice())?;
    task.put_con_bound(0i32,mosek::Boundkey::FX,w0,w0)?;

    // cardinality constraint
    task.put_con_name(1,"cardinality")?;
    task.put_a_row(1,
                   y,
                   n_ones.as_slice())?;
    task.put_con_bound(1,mosek::Boundkey::UP,p as f64,p as f64)?;

    // (gamma,G'x) in Q
    {
        let afei = task.get_num_afe()?;
        let acci = task.get_num_acc()?;

        task.append_afes(k as i64+1)?;
        let dom = task.append_quadratic_cone_domain(k as i64+1)?;
        task.append_acc_seq(dom,
                            afei,
                            vec![0.0; k as usize + 1].as_slice())?;
        task.put_acc_name(acci,"GT")?;
        task.put_afe_g(afei,gamma)?;

        for ((i,j),v) in iproduct!(0..n,0..n).zip(GT).filter(|(_,v)| **v != 0.0) {
            task.put_afe_f_entry(afei + i as i64 + 1, j as i32, *v)?;
        }
    }

    //DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
    {
        task.append_djcs(n as i64)?;
        let domeq = task.append_rzero_domain(1)?;
        for (i,xi,yi,x0i) in izip!(0..n,x,y,x0) {
            let afei = task.get_num_afe()?;
            task.append_afes(3)?;
            // y_j = 0
            task.put_afe_f_entry(afei,*yi,1.0)?;
            // x_j = x0_j
            task.put_afe_f_entry(afei+1,*xi,1.0)?;
            // y_j = 1
            task.put_afe_f_entry(afei+2,*yi,1.0)?;
            task.put_djc(i as i64,
                         &[domeq,domeq,domeq],
                         &[afei,afei+1,afei+2],
                         &[0.0, *x0i, 1.0],
                         &[2,1])?;
            task.put_djc_name(i as i64,format!("y[{}]->x[{}]>0",i,i).as_str())?;
        }
    }

    let _ = task.optimize()?;
    // task.write_data(format!("pf-card-{}.ptf",p).as_str())?;

    let mut xx = vec![0.0;n as usize];
    task.get_xx_slice(Soltype::ITG, 0,n,xx.as_mut_slice())?;
    Ok((xx[0..n as usize].to_vec(),task.get_primal_obj(Soltype::ITG)?))
}


// #[allow(non_snake_case)]
fn main() -> Result<(),String> {
    for p in 1..4 {
        let (x,expret) = portfolio(common::Data1::n,
                                   common::Data1::mu,
                                   common::Data1::GT,
                                   common::Data1::x0,
                                   common::Data1::gamma,
                                   p,
                                   common::Data1::w)?;
        println!("Bound {}: x = {:?}", p,x);
        println!("  Return: {:.5e}\n", expret);
    }
    Ok(())
}

