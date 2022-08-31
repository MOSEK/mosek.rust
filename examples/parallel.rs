//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : parallel.rs
//!
//!   Purpose: Demonstrates parallel optimization using optimizebatch()
//!
extern crate mosek;
extern crate itertools;

use mosek::{Task};
use std::env;
use itertools::{izip};

/// Example of how to use env.optimize_batch().
/// Optimizes tasks whose names were read from command line.
fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Syntax: parallel FILENAME FILENAME [ FILENAME ... ]");
        Err("Invalid argument list".to_string())
    }
    else {
        // Create an example list of tasks to optimize
        let mut tasks : Vec<(String,Task)> = args[1..].iter().filter_map(|fname| {
            let mut t = Task::new().unwrap();
            if let Err(_) = t.read_data(fname.as_str()) { None }
            else {
                t.put_int_param(mosek::Iparam::NUM_THREADS, 2).unwrap();
                Some((fname.as_str().to_string(),t))
            }
        }).collect();

        let mut res = vec![0i32; tasks.len()];
        let mut trm = vec![0i32; tasks.len()];
        {
            let taskrs : Vec<& mut Task> = tasks.iter_mut().map(|(_a,b)| b).collect();

            // Size of thread pool available for all tasks
            let threadpoolsize : i32 = 6;

            // Optimize all the given tasks in parallel
                mosek::optimize_batch(false,          // No race
                                      -1.0,           // No time limit
                                      threadpoolsize,
                                      taskrs.as_slice(),          // Array of tasks to optimize
                                      trm.as_mut_slice(),
                                      res.as_mut_slice())?;
        }

        for (resi,trmi,(fname,ti)) in izip!(res,trm,tasks.iter()) {
            println!("Task  {}  res {}   trm {}   obj_val  {}  time {}",
                     fname,
                     resi,
                     trmi,
                     ti.get_dou_inf(mosek::Dinfitem::INTPNT_PRIMAL_OBJ)?,
                     ti.get_dou_inf(mosek::Dinfitem::OPTIMIZER_TIME)?);
        }
        Ok(())
    }
}
