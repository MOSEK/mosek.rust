//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! File : qcqo1.rs
//!
//! Purpose :   Demonstrate how to solve a quadratic
//!             optimization problem using the MOSEK API.
//!
//!             minimize  x0^2 + 0.1 x1^2 +  x2^2 - x0 x2 - x1
//!             s.t
//!                       1 <=  x0 + x1 + x2 - x0^2 - x1^2 - 0.1 x2^2 + 0.2 x0 x2
//!                       x >= 0

extern crate mosek;
extern crate itertools;
use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Soltype};
use itertools::izip;

const INF : f64 = 0.0;


fn main() -> Result<(),String> {
    const NUMCON : i32 = 1;   /* Number of constraints.             */
    const NUMVAR : i32 = 3;   /* Number of variables.               */

    let c = [0.0, -1.0, 0.0];

    let bkc  = [Boundkey::LO];
    let blc = [1.0];
    let buc = [INF];

    let bkx = [Boundkey::LO,
               Boundkey::LO,
               Boundkey::LO ];
    let blx = [0.0,
               0.0,
               0.0 ];
    let bux = [INF,
               INF,
               INF ];

    let asub = [ &[0i32], &[0i32], &[0i32] ];
    let aval = [ &[1.0], &[1.0], &[1.0] ];

    let mut task = Task::new().unwrap().with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // Give MOSEK an estimate of the size of the input data.
    // This is done to increase the speed of inputting data.
    // However, it is optional.
    // Append 'numcon' empty constraints.
    // The constraints will initially have no bounds.
    task.append_cons(NUMCON)?;

    // Append 'numvar' variables.
    // The variables will initially be fixed at zero (x=0).
    task.append_vars(NUMVAR)?;

    for (j,cj,bkj,blj,buj) in izip!(0..NUMVAR,c,bkx,blx,bux) {
        // Set the linear term c_j in the objective.
        task.put_c_j(j, cj)?;
        // Set the bounds on variable j.
        // blx[j] <= x_j <= bux[j]
        task.put_var_bound(j, bkj, blj, buj)?;
    }

    for (j,asubj,avalj) in izip!(0..NUMVAR, asub, aval) {
        /* Input column j of A */
        task.put_a_col(j,                     /* Variable (column) index.*/
                       asubj,               /* Row index of non-zeros in column j.*/
                       avalj)?;              /* Non-zero Values of column j. */
    }
    // Set the bounds on constraints.
    // for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] 
    for (i,bki,bli,bui) in izip!(0..NUMCON,bkc,blc,buc) {
        task.put_con_bound(i, bki, bli, bui)?;
    }

    {
        // The lower triangular part of the Q
        // matrix in the objective is specified.
        let qosubi = &[ 0,   1,   2,    2 ];
        let qosubj = &[ 0,   1,   0,    2 ];
        let qoval  = &[ 2.0, 0.2, -1.0, 2.0 ];
        // Input the Q for the objective.

        task.put_q_obj(qosubi, qosubj, qoval)?;
    }

    // The lower triangular part of the Q^0
    // matrix in the first constraint is specified.
    // This corresponds to adding the term
    // x0^2 - x1^2 - 0.1 x2^2 + 0.2 x0 x2

    {
      let qsubi = &[0,   1,    2,   2  ];
      let qsubj = &[0,   1,    2,   0  ];
      let qval =  &[-2.0, -2.0, -0.2, 0.2];

      /* put Q^0 in constraint with index 0. */

      task.put_q_con_k(0,
                       qsubi,
                       qsubj,
                       qval)?;
    }

    task.put_obj_sense(Objsense::MINIMIZE)?;

    /* Solve the problem */

    let _trm = task.optimize()?;

    // Print a summary containing information
    //   about the solution for debugging purposes
    task.solution_summary(Streamtype::MSG)?;

    /* Get status information about the solution */
    let solsta = task.get_sol_sta(Soltype::ITR)?;

    let mut xx = vec![0.0; NUMVAR as usize];
    task.get_xx(Soltype::ITR, // Interior solution.
                 xx.as_mut_slice())?;

    match solsta {
        Solsta::OPTIMAL => {
            println!("Optimal primal solution");
            for (j,xj) in izip!(0..NUMVAR,xx) {
                println!("x[{}]: {}",j,xj);
            }
        },
        Solsta::DUAL_INFEAS_CER|
        Solsta::PRIM_INFEAS_CER =>
          println!("Primal or dual infeasibility.\n"),
        Solsta::UNKNOWN =>
            println!("Unknown solution status.\n"),
        _ =>
            println!("Other solution status")
    }
    Ok(())
} /* Main */

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
