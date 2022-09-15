//
//   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//
//   File :      parameters.rs
//
//   Purpose :   Demonstrates a very simple example about how to get/set
//               parameters with MOSEK Julia API
//

extern crate mosek;

use mosek::{Task,Iparam,Dparam,Dinfitem,Iinfitem,Optimizertype,Basindtype};

fn main() -> Result<(),String> {
    let mut task = Task::new().unwrap();
    println!("Test MOSEK parameter get/set functions");

    // Set log level (integer parameter)
    task.put_int_param(Iparam::LOG, 1)?;
    // Select interior-point optimizer... (integer parameter)
    task.put_int_param(Iparam::OPTIMIZER, Optimizertype::INTPNT)?;
    // ... without basis identification (integer parameter)
    task.put_int_param(Iparam::INTPNT_BASIS,Basindtype::NEVER)?;
    // Set relative gap tolerance (double parameter)
    task.put_dou_param(Dparam::INTPNT_CO_TOL_REL_GAP, 1.0e-7)?;

    // The same using explicit string names
    task.put_param("MSK_DPAR_INTPNT_CO_TOL_REL_GAP", "1.0e-7")?;
    task.put_na_dou_param("MSK_DPAR_INTPNT_CO_TOL_REL_GAP",  1.0e-7 )?;

    // Incorrect value

    if let Err(_) = task.put_dou_param(Dparam::INTPNT_CO_TOL_REL_GAP, -1.0) {
        println!("Wrong parameter value");
    }


    let param = task.get_dou_param(Dparam::INTPNT_CO_TOL_REL_GAP)?;
    println!("Current value for parameter intpnt_co_tol_rel_gap = $param");

    // Define and solve an optimization problem here
    // optimize(task,)
    // After optimization:

    println!("Get MOSEK information items");

    let tm = task.get_dou_inf(Dinfitem::OPTIMIZER_TIME)?;
    let iter = task.get_int_inf(Iinfitem::INTPNT_ITER)?;

    println!("Time: {}",tm);
    println!("Iterations: {}",iter);
    Ok(())
}
