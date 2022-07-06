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

    task.append_vars(3*n)?;

    let all_vars : Vec<i32> = (0..2*n).collect();
    let x = &all_vars[0..n as usize];
    let y = &all_vars[n as usize..2*n as usize];
    let z = &all_vars[2*n as usize..3*n as usize];

    task.put_var_bound_slice_const(0,n,mosek::Boundkey::LO,0.0,0.0)?;
    task.put_var_bound_slice_const(n,2*n,mosek::Boundkey::RA,0.0,1.0)?;
    task.put_var_bound_slice_const(2*n,3*n, mosek::Boundkey::RA, 0.0,1.0)?;

    for (i,xj,yj,zj) in izip!(0..n,x,y,z) {
        task.put_var_name(*xj,format!("x[{}]",i).as_str())?;
        task.put_var_name(*yj,format!("y[{}]",i).as_str())?;
        task.put_var_name(*zj,format!("z[{}]",i).as_str())?;
        task.put_var_type(*yj, Variabletype::TYPE_INT);
    }

    // objective
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for (j,mui) in x.iter().zip(mu.iter()) {
        task.put_c_j(*j, *mui)?;
    }

    let n_ones = vec![1.0; n as usize];
    // budget constraint
    {
        let coni = task.getnumcon()?;
        task.append_cons(1)?;
        task.put_con_name(coni,"budget")?;
        task.put_a_row(coni,
                       x,
                       n_ones.as_slice())?;
        task.put_con_bound(coni,mosek::Boundkey::FX,w0,w0)?;
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
    
    // cardinality constraint
    {
        let coni = task.getnumcon()?;
        task.append_cons(1)?;
        task.put_con_name(coi,"cardinality")?;
        task.put_a_row(coni, y, n_ones.as_slice())?;
        task.put_con_bound(coni,mosek::Boundkey::UP,p as f64,p as f64)?;
    }
    
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

