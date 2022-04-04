// File : portfolio_1_basic.rs

// Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.

// Description :  Implements a basic portfolio optimization model.
//
// Maximize c'x
// Such That [ gamma, G'x ] in qcone

extern crate mosek;

fn portfolio(n : i32,
             gamma : f64,
             mu : &[f64],
             GT : &[f64],
             x0 : &[f64],
             w : f64) -> Result<Vec<f64>,String> {

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
    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;

    /* Total budget */
    let total_budget = w + x0.iter().sum::<f64>();

    /* Constraints. */
    task.append_cons(1i32)?;
    /* Variables. */
    task.append_vars(1i32+n)?;

    let x : Vec<i32> = (0i32..n).collect();
    let s : Vec<i32> = vec![n];

    /* Total budget constraint - set bounds l^c = u^c */
    task.put_con_bound(0i32, mosek::MSK_BK_FX, total_budget, total_budget)?;
    task.put_con_name(0i32,"budget")?;

    /* x variables. */
    for j in 0..n as usize {
        /* Return of asset j in the objective */
        task.put_c_j(x[j], mu[j])?;
        /* Coefficients in the first row of A */
        task.put_aij(0, x[j], 1.0)?;
        /* No short-selling - x^l = 0, x^u = inf */
        task.put_var_bound(x[j], mosek::MSK_BK_LO, 0.0, 0.0)?;
        task.put_var_name(x[j], format!("x[{}]",j+1).as_str())?;
    }

    /* s variable is a constant equal to gamma. */
    task.put_var_bound(s[0], mosek::MSK_BK_FX, gamma, gamma)?;
    task.put_var_name(s[0], "s")?;

    /* Define the cone spanned by variables (s, t), i.e. dimension = n + 1 */
    /* (s >= GT*x). */
    task.append_afes(n as i64+1)?;
    for j in 0..n {
        /* Copying the GT matrix in the appropriate block of A */
        for k in 0..n {
            if  GT[(k*n+j) as usize] != 0.0  {
                task.put_afe_f_entry(k as i64, x[j as usize], GT[(k*n+j) as usize])?;
            }
        }
    }

    let dom = task.append_quadratic_cone_domain(n as i64+1)?;
    task.append_acc(dom,
                    (0..n as i64+1).collect::<Vec<i64>>().as_slice(),
                    (0..n+1).map(|_| 0.0).collect::<Vec<f64>>().as_slice())?;

    task.put_acc_name(0,"stddev");


    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE);

    /* Dump the problem to a human readable OPF file. */
    task.write_data("dump.ptf");

    let trm = task.optimize()?;

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(mosek::MSK_STREAM_MSG)?;

    /* Read the x variables one by one and compute expected return. */
    /* Can also be obtained as value of the objective. */
    let mut level = vec![0.0;n as usize + 1];
    task.get_xx_slice(mosek::MSK_SOL_ITR,0,n+1,level.as_mut_slice())?;

    let xx = &level[0..n as usize];

    let expret : f64 = xx.iter().zip(mu.iter()).map(|(a,b)| a*b).sum();

    /* Get the value of s. This should be gamma. */
    let stddev = level[s[0] as usize];
    println!("\nExpected return {:.4e} for gamma {:.4e}\n", expret, stddev);
    Ok(xx.to_vec())
}




fn main() -> Result<(),String> {
    let expret = 0.0;
    let stddev = 0.0;

    let n : i32 = 3;
    let gamma = 0.05;
    let mu = vec![0.1073,  0.0737,  0.0627];
    let GT = vec![0.1667,  0.0232,  0.0013 ,
                  0.0000,  0.1033, -0.0022 ,
                  0.0000,  0.0000,  0.0338];
    let x0 = vec![0.0, 0.0, 0.0];
    let w = 1.0;

    let _ = portfolio(n,gamma,mu.as_slice(),GT.as_slice(),x0.as_slice(),w)?;

    return Ok(());
}



// let m = Mosek::named("Molly")?;
// let x = m.named_variable("x",Domain::greater_than(vec![0.0; n].as_slice()))?;
// m.named_objective("obj",mosek::Maximize, Expr::dot(x,mu))?;
// let t = m.named_variable("t",Domain::equal_toscalar(gamma))?;
// let _ = m.named_constraint("wealth",Expr::sum(x.as_expr()),Domain::equal_to(total_budget))?;
// let _ = m.named_constraint("GTx",Expr::vstack(t.as_expr(),Expr::mul(GT,x),Domain::in_quadratic_cone(n+1)))?;
// m.solve()?;
// let xlevel = m.get_level(x);
