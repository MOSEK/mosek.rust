
// File : portfolio_1_basic.rs

// Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.

// Description :  Implements a basic portfolio optimization model.
//                Determines points on the efficient frontier.
// Maximize mu'*x - alpha * s
// Subject to
//     budget : sum(x)  = 1
//     GT     : (0.5,s,G'*x) in Q_r^{n+2}
//              == (s >= |G'x|
//     x >= 0

extern crate mosek;

fn portfolio(n : i32,
             mu : &[f64],
             GT : &[f64],
             x0  : &[f64],
             alphas : &[f64],
             w : f64) -> Result<(),String> {
    /* Initial setup. */
    let env = match mosek::Env::new() {
        Some(e) => e,
        None => return Err("Failed to create env".to_string()),
    };
    /* Create the optimization task. */
    let mut task = match env.task() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };
    //task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;

    task.append_vars(n+1)?;
    task.append_cons(1)?;

    /* Objective */
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE)?;
    for (j,mu_j) in mu.iter().enumerate() {
        task.put_c_j(j as i32,*mu_j);
        task.put_aij(0,j as i32,1.0);
        task.put_var_bound(j as i32, mosek::MSK_BK_LO, 0.0,0.0);
        task.put_var_name(j as i32, format!("x[{}]",j).as_str());
    }
    task.put_var_name(n, "s")?;
    task.put_var_bound(n, mosek::MSK_BK_FR, 0.0, 0.0);
    /* Total budget */
    let total_budget = w + x0.iter().sum::<f64>();
    task.put_con_name(0,"budget");

    /**/
    task.append_afes(n as i64+2);
    let dom = task.append_r_quadratic_cone_domain(n as i64+2)?;
    task.append_acc(dom,
                    (0..n as i64+2).collect::<Vec<i64>>().as_slice(),
                    (0..n+2).map(|_| 0.0).collect::<Vec<f64>>().as_slice())?;
    {
        task.put_afe_g(0,0.5);
        task.put_afe_f_entry(1,n,1.0);
        let mut k = 0;
        for i in 0..n {
            for j in 0..n {
                if GT[k] != 0.0 {
                    task.put_afe_f_entry(i as i64+2,j as i32,GT[k])?;
                }
                k += 1;
            }
        }
    }

    for (i,alpha) in alphas.iter().enumerate() {
        /* Sets the objective function coefficient for s. */
        task.put_c_j(n, -alpha)?;
        let trmcode = task.optimize()?;
        let solsta = task.get_sol_sta(mosek::MSK_SOL_ITR)?;

        if solsta == mosek::MSK_SOL_STA_OPTIMAL {
            let mut xx = vec![0.0; n as usize+1];

            task.get_xx(mosek::MSK_SOL_ITR,xx.as_mut_slice())?;
            let expret : f64 = xx[0..n as usize].iter().sum();
            let stddev = xx[n as usize];
            println!("alpha:{:-12.3e} | expected return:{:-12.3e} | std dev:{:-12.3e}\n", alpha, expret, stddev);
        }
        else {
            println!("An error occurred when solving for alpha={:e}\n", alpha);
        }
        task.write_data(format!("portfolio-2-{}.ptf",i).as_str())?;
    }

    
    Ok(())
}

fn main() -> Result<(),String> {
    let n : i32 = 3;
    let mu = vec![0.1073,  0.0737,  0.0627];
    let x0 = vec![0.0, 0.0, 0.0];
    let w = 1.0;
    let alphas = vec![0.0, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.0, 4.5];
    let GT = vec![0.1667,  0.0232,  0.0013,
                  0.0000,  0.1033, -0.0022,
                  0.0000,  0.0000,  0.0338];

    portfolio(n,mu.as_slice(),GT.as_slice(),x0.as_slice(),alphas.as_slice(),w)
}
