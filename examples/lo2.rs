// Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//
// File :      lo2.rs
//
// Purpose :   Demonstrates how to solve a small linear
//             optimization problem using the MOSEK Java API.

extern crate mosek;
extern crate itertools;

use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Soltype};
use itertools::izip;

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    let numcon : i32 = 3;
    let numvar : i32 = 4;

    let c    = [3.0, 1.0, 5.0, 1.0];
    let asub = [0, 1, 2,
                0, 1, 2, 3,
                1, 3];
    let aval = [3.0, 1.0, 2.0,
                2.0, 1.0, 3.0, 1.0,
                2.0, 3.0];
    let aptr = [0,3,7,9];
    let bkc  = [Boundkey::FX,
                Boundkey::LO,
                Boundkey::UP];

    let blc = [30.0,
               15.0,
               -INF];
    let buc  = [30.0,
                INF,
                25.0];
    let bkx  = [Boundkey::LO,
                Boundkey::RA,
                Boundkey::LO,
                Boundkey::LO];
    let blx  = [0.0,
                0.0,
                0.0,
                0.0];
    let bux  = [INF,
                10.0,
                INF,
                INF];

    // Create a task object linked with the environment env.
    let mut task = Task::new().unwrap().with_callbacks();
    // Directs the log task stream to the user specified
    // method task_msg_obj.streamCB
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Give MOSEK an estimate of the size of the input data.
    This is done to increase the speed of inputting data.
    However, it is optional. */
    /* Append 'numcon' empty constraints.
    The constraints will initially have no bounds. */
    task.append_cons(numcon)?;

        /* Append 'numvar' variables.
    The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar)?;

    /* Optionally add a constant term to the objective. */
    task.put_cfix(0.0)?;

    for (j,(&cj,&bkj,&blj,&buj)) in izip!(c.iter(),bkx.iter(),blx.iter(),bux.iter()).enumerate() {
        /* Set the linear term c_j in the objective.*/
        task.put_c_j(j as i32, cj)?;
        /* Set the bounds on variable j.
        blx[j] <= x_j <= bux[j] */
        task.put_var_bound(j as i32, bkj, blj, buj)?;
    };
    /* Set the bounds on constraints.
    for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] */
    for (i,(&bki,&bli,&bui,&ptrb,&ptre)) in izip!(bkc.iter(),blc.iter(),buc.iter(),aptr[..aptr.len()-1].iter(),aptr[1..].iter()).enumerate() {
        task.put_con_bound(i as i32, bki, bli, bui)?;
        /* Input row i of A */
        task.put_a_row(i as i32,                        /* Row index.*/
                       &asub[ptrb..ptre],               /* Column indexes of non-zeros in row i.*/
                       &aval[ptrb..ptre])?;              /* Non-zero Values of row i. */
    }

    task.put_obj_sense(Objsense::MAXIMIZE)?;
    task.optimize()?;

    // Print a summary containing information
    //   about the solution for debugging purposes
    task.solution_summary(Streamtype::MSG)?;

    /* Get status information about the solution */
    let solsta = task.get_sol_sta(Soltype::BAS)?;
    let mut xx = vec![0.0; numvar as usize];
    task.get_xx(Soltype::BAS, // Basic solution.
                xx.as_mut_slice());

    match solsta {
        Solsta::OPTIMAL =>
            println!("Optimal primal solution = {:?}",xx),
        Solsta::DUAL_INFEAS_CER|Solsta::PRIM_INFEAS_CER =>
            println!("Primal or dual infeasibility."),
        Solsta::UNKNOWN =>
            println!("Unknown solution status."),
        _ =>
            println!("Other solution status")
    }

    Ok(())
}
