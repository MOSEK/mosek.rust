/*
   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.

   File :      $${file}

   Purpose :   Demonstrates how to solve a small mixed
               integer conic optimization problem.

               minimize    x^2 + y^2
               subject to  x >= e^y + 3.8
                           x, y - integer
*/
extern crate mosek;

fn main() -> Result<(),String> {
    let env = match mosek::Env::new() {
        Some(e) => e,
        None => return Err("Failed to create env".to_string()),
        };
    /* Create the optimization task. */
    let mut task = match env.task() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };
    let infinity = 0.0; // for symbolic use, value is irrelevant

    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;

    task.append_vars(6)?;
    task.append_cons(3)?;
    task.put_var_bound_slice_const(0, 6, mosek::MSK_BK_FR, -infinity, infinity)?;

    // Integrality constraints
    task.put_var_type_list(vec![1i32,2i32].as_slice(),
                           vec![mosek::MSK_VAR_TYPE_INT, mosek::MSK_VAR_TYPE_INT].as_slice())?;

    // Set up the three auxiliary linear constraints
    task.put_aij_list(vec![0i32,0i32,1i32,2i32,2i32].as_slice(),
                      vec![1i32,3i32,4i32,2i32,5i32].as_slice(),
                      vec![-1.0,1.0,1.0,1.0,-1.0].as_slice())?;
    task.put_con_bound_slice(0, 3, 
                             vec![mosek::MSK_BK_FX, mosek::MSK_BK_FX, mosek::MSK_BK_FX].as_slice(),
                             vec![-3.8, 1.0, 0.0].as_slice(),
                             vec![-3.8, 1.0, 0.0].as_slice())?;

    // Objective
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MINIMIZE)?;
    task.put_c_j(0, 1.0)?;

    // Conic part of the problem
    task.append_afes(6)?;
    for i in 0..6 {
        task.put_afe_f_entry(i as i64, i as i32, 1.0)?;
    }
    {
        let domidx = task.append_quadratic_cone_domain(3)?;
        task.append_acc(domidx,
                        vec![0i64,1i64,2i64].as_slice(),
                        vec![0.0,0.0,0.0].as_slice())?;
    }
    {
        let domidx = task.append_primal_exp_cone_domain()?;
        task.append_acc(domidx,vec![3i64,4i64,5i64].as_slice(),vec![0.0,0.0,0.0].as_slice())?;
    }
    // Optimize the task
    let _trm = task.optimize()?;
    task.solution_summary(mosek::MSK_STREAM_MSG)?;

    let mut xx = vec![0.0; 2];
    task.get_xx_slice(mosek::MSK_SOL_ITG, 1, 3, xx.as_mut_slice())?;
    println!("x = {}  y = {}",xx[0],xx[1]);
    Ok(())
}