// File : $${file}

// Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.

// Description :  Implements a basic portfolio optimization model
//                with transaction costs of order x^(3/2).
//
// Maximize mu'x
// Subject to
//    budget : sum(x)+m't = sum(x0)+w
//    GT     : (gamma,G'x) in Q^{k+1}
//    MI     : (t_j,1,|x_j-x0_j|) in P^3(2/3,1/3), j = 1..
//    x >= 0
//
// Where
//    m_i is the transaction cost associated with asset i
//    gamma is the bound on the standard deviation if the portfolio
//    mu_i is the expected return on asset i
//    w is the initial wealth held in cash
//    x0_i is the initial investment in asset i
//    G'G is the covariance matrix for assets
extern crate mosek;
use mosek::{Task,Objsense,Streamtype,Solsta,Soltype};

#[allow(non_snake_case)]
fn portfolio(n : i32,
             mu : &[f64],
             m  : &[f64],
             GT : &[f64],
             x0  : &[f64],
             gamma : f64,
             w : f64) -> Result<(),String> {

    let k = (GT.len() / n as usize) as i32;
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    task.append_vars(3*n)?;
    for i in 0..n {
        task.put_var_bound(i,mosek::Boundkey::LO, 0.0, 0.0)?;
        task.put_var_name(i,format!("x[{}]",i).as_str())?; }
    for i in n..2*n {
        task.put_var_bound(i,mosek::Boundkey::FR, 0.0, 0.0)?;
        task.put_var_name(i,format!("c[{}]",i).as_str())?;
    }
    for i in 2*n..3*n {
        task.put_var_bound(i,mosek::Boundkey::FR, 0.0, 0.0)?;
        task.put_var_name(i,format!("z[{}]",i).as_str())?;
    }

    
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for i in 0..n {
        task.put_c_j(i,mu[i as usize])?;
    }

    task.append_cons(1+2*n)?;

    // budget
    task.put_con_name(0,"budget")?;
    let wealth = w + x0.iter().sum::<f64>();
    task.put_a_row(0,
                   (0..2*n).collect::<Vec<i32>>().as_slice(),
                   (0..n).map(|_| 1.0).chain(m.iter().map(|v| *v)).collect::<Vec<f64>>().as_slice())?;
    task.put_con_bound(0,mosek::Boundkey::FX, wealth,wealth)?;

    // zpos
    let z_base = 2*n;
    for i in 0..n {
        // x - z < x0
        task.put_a_row(1+i,vec![i, z_base+i].as_slice(), vec![1.0,-1.0].as_slice())?;
        task.put_con_bound(1+i, mosek::Boundkey::LO, x0[i as usize], 0.0)?;

        // z - x < x0
        task.put_a_row(1+n+i,vec![i, z_base+i].as_slice(), vec![-1.0,1.0].as_slice())?;
        task.put_con_bound(1+n+i, mosek::Boundkey::LO, x0[i as usize], 0.0)?;
    }

    // GT
    {
        let acci = task.get_num_acc()?;
        let afei = task.get_num_afe()?;

        task.append_afes(k as i64 + 1)?;
        let dom = task.append_quadratic_cone_domain(k as i64+1)?;
        task.append_acc(dom,
                        (afei..afei+k as i64+1).collect::<Vec<i64>>().as_slice(),
                        (0..k+1).map(|_| 0.0).collect::<Vec<f64>>().as_slice())?;
        task.put_acc_name(acci,"GT")?;
        task.put_afe_g(0,gamma)?;
        let mut l = 0;
        for i in 0..k {
            for j in 0..n {
                if GT[l] != 0.0 {
                    task.put_afe_f_entry(i as i64+1,j as i32,GT[l])?;
                }
                l += 1;
            }
        }
    }
    // MI
    {
        let mut acci = task.get_num_acc()?;
        let mut afei = task.get_num_afe()?;
        task.append_afes(n as i64 * 3)?;
        let dom = task.append_primal_power_cone_domain(3,&[2.0/3.0, 1.0/3.0])?;

        for i in 0..n {
            task.put_afe_f_entry(afei,n+i,1.0)?;
            task.put_afe_g(afei+1,1.0)?;
            task.put_afe_f_entry(afei+2,z_base+i,1.0)?;
            task.put_afe_g(afei+2,-x0[i as usize])?;
            task.append_acc(dom,
                            (afei..afei+3).collect::<Vec<i64>>().as_slice(),
                            (0..3).map(|_| 0.0).collect::<Vec<f64>>().as_slice())?;
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

    println!("Solution x = {:?}",level);

    Ok(())
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n     = 3i32;
    let w     = 1.0;
    let m     = vec![0.01, 0.01, 0.01];
    let x0    = vec![0.0, 0.0, 0.0];
    let gamma = 0.05;
    let mu    = vec![0.1073,  0.0737,  0.0627];
    let GT    = vec![0.1667,  0.0232,  0.0013,
                     0.0000,  0.1033, -0.0022,
                     0.0000,  0.0000,  0.0338];

    portfolio(n, mu.as_slice(), m.as_slice(), GT.as_slice(), x0.as_slice(), gamma, w)?;
    Ok(())
}
