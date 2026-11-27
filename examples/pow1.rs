//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : pow1.rs
//!
//!  Purpose: Demonstrates how to solve the problem
//!
//!    maximize x^0.2*y^0.8 + z^0.4 - x
//!          st x + y + 0.5z = 2
//!             x,y,z >= 0
//!

extern crate mosek;
use mosek::*;

const INF : f64 = 0.0;

fn main()  -> Result<(),String> {
    let numcon : i32 = 1;
    let numvar : i32 = 5;

    // Since the value infinity is never used, we define
    // 'infinity' symbolic purposes only

    let cval = vec![ 1.0, 1.0, -1.0 ];
    let csub = vec![ 3,   4,    0 ];

    let aval = vec![ 1.0, 1.0, 0.5 ];
    let asub = vec![ 0, 1, 2 ];

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    }.with_callbacks();
    // Directs the log task stream to the user specified
    // method msgclass.streamCB
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Append 'numcon' empty constraints.
    The constraints will initially have no bounds. */
    task.append_cons(numcon)?;

    /* Append 'numvar' variables.
    The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar)?;

    /* Set up the linear part of the problem */
    task.put_c_list(&csub, &cval)?;
    task.put_a_row(0, &asub, &aval)?;
    task.put_con_bound(0, Boundkey::FX, 2.0, 2.0)?;


    task.put_var_bound_slice_const(0, numvar, Boundkey::FR, -INF, INF)?;

    /* Add a conic constraint */
    let pc1 = task.append_primal_power_cone_domain(3, &[0.2, 0.8])?;
    let pc2 = task.append_primal_power_cone_domain(3, &[4.0, 6.0])?;

    // Create data structures F,g so that
    //
    //   F * x + g = (x(0), x(1), x(3), x(2), 1.0, x(4))
    //
    task.append_afes(6)?;
    task.put_afe_f_entry_list(&[0, 1, 2, 3, 5],         // Rows
                              &[0, 1, 3, 2, 4],         // Columns
                              &[1.0, 1.0, 1.0, 1.0, 1.0])?;
    task.put_afe_g(4, 1.0)?;

    // Append the two conic constraints
    task.append_acc(pc1,                  // Domain
                    &[0, 1, 2],           // Rows from F
                    &[0.0,0.0,0.0])?;     // Unused
    task.append_acc(pc2,                  // Domain
                    &[3, 4, 5],           // Rows from F
                    &[0.0,0.0,0.0])?;     // Unused

    task.put_obj_sense(Objsense::MAXIMIZE)?;
    task.optimize()?;

    task.write_data("pow1.ptf")?;
    // Print a summary containing information
    // about the solution for debugging purposes
    task.solution_summary(Streamtype::LOG)?;
    /* Get status information about the solution */
    let solsta = task.get_sol_sta(Soltype::ITR)?;

    assert!(solsta == Solsta::OPTIMAL);

    let mut xx = vec![0.0; numvar as usize];
    task.get_xx(Soltype::ITR,
                xx.as_mut_slice())?;

    println!("Optimal primal solution");
    for (j,&xj) in xx[0..3].iter().enumerate() {
        println!("x[{}]: {}",j+1,xj);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
