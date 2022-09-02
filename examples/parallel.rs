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
enum FileOrText {
    File(String),
    Text(String)
}
fn main() -> Result<(),String> {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Syntax: parallel FILENAME FILENAME [ FILENAME ... ]");
        Err("Invalid argument list".to_string())
    }
    else {
        let _ = args.next();
        parallel(args.map(|s| FileOrText::File(s)).collect())
    }
}
fn parallel(files : Vec<FileOrText>) -> Result<(),String> {
    // Create an example list of tasks to optimize
    let mut tasks : Vec<(String,Task)> = files.iter().filter_map(|fname| {
        let mut t = Task::new().unwrap();
        match fname {
            FileOrText::File(fname) => {
                if let Err(_) = t.read_data(fname.as_str()) { None }
                else {
                    t.put_int_param(mosek::Iparam::NUM_THREADS, 2).unwrap();
                    Some((fname.as_str().to_string(),t))
                }
            },
            FileOrText::Text(data) => {
                if let Err(_) = t.read_ptf_string(data.as_str()) { None }
                else {
                    t.put_int_param(mosek::Iparam::NUM_THREADS, 2).unwrap();
                    Some(("-".to_string(),t))
                }
            }
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



#[cfg(test)]
mod tests {

    const DFLT_FILE1 : &str = "Task
Objective
    Maximize + 2 @x0 + 3 @x1 - @x2
Constraints
    @c0 [1] + @x0 + @x1 + @x2
    @C0 [QUAD(3)]
        @ac1: + 0.03
        @ac2: + 1.5 @x0 + 0.1 @x1
        @ac3: + 0.3 @x0 + 2.1 @x2 + 0.1
Variables
    @x0
    @x1
    @x2
";

    const DFLT_FILE2 : &str = "Task
Objective
    Maximize + @x0 + 0.64 @x1
Constraints
    @c0 [-inf;250] + 50 @x0 + 31 @x1
    @c1 [-4;+inf] + 3 @x0 - 2 @x1
Variables
    @x0[0;+inf]
    @x1[0;+inf]
Integers
    @x0 @x1
";
    #[test]
    fn test() {
        super::parallel(vec![super::FileOrText::Text(DFLT_FILE1.to_string()),
                             super::FileOrText::Text(DFLT_FILE2.to_string())]).unwrap();
    }
}
