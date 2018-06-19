/*
  Copyright: MOSEK ApS

  Purpose: Demonstrates how to solve the problem

    maximize x^0.2*y^0.8 + z^0.4 - x
          st x + y + 0.5z = 2
             x,y,z >= 0
*/

extern crate mosek;
use mosek::*;

const INFTY : f64 = 0.0;

fn main()
{
    let numcon : i32 = 1;
    let numvar : i32 = 6;

    // Since the value infinity is never used, we define
    // 'infinity' symbolic purposes only

    let val = vec![ 1.0, 1.0, -1.0 ];
    let sub = vec![ 3,   4,    0 ];

    let aval = vec![ 1.0, 1.0, 0.5 ];
    let asub = vec![ 0, 1, 2 ];

    // Make mosek environment.
    let env = Env::new();
    let mut task = env.task();
    // Directs the log task stream to the user specified
    // method msgclass.streamCB
    task.put_stream_callback(MSK_STREAM_LOG, |msg| print!("{}",msg));
    /*TAG:end-maketask*/

    /* Append 'numcon' empty constraints.
    The constraints will initially have no bounds. */
    task.append_cons(numcon);

    /* Append 'numvar' variables.
    The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar);

    /* Set up the linear part of the problem */
    task.put_c_list(&sub, &val);
    task.put_a_row(0, &asub, &aval);
    task.put_con_bound(0, MSK_BK_FX, 2.0, 2.0);
    

    let bkx = vec![MSK_BK_FR, MSK_BK_FR, MSK_BK_FR, MSK_BK_FR, MSK_BK_FR, MSK_BK_FX];
    let blx = vec![-INFTY,    -INFTY,    -INFTY,    -INFTY,    -INFTY,    1.0      ];
    let bux = vec![ INFTY,     INFTY,     INFTY,     INFTY,     INFTY,    1.0      ];
    task.put_var_bound_slice(0, numvar, &bkx, &blx, &bux);

    /* Add a conic constraint */
    task.append_cone(MSK_CT_PPOW, 0.2, &vec![0, 1, 3]);
    task.append_cone(MSK_CT_PPOW, 0.4, &vec![2, 5, 4]);
    //TAG:end-appendcone
    
    task.put_obj_sense(MSK_OBJECTIVE_SENSE_MAXIMIZE);
    task.optimize();

    // Print a summary containing information
    // about the solution for debugging purposes
    task.solution_summary(MSK_STREAM_LOG);
    /* Get status information about the solution */
    let solsta = task.get_sol_sta(MSK_SOL_ITR);



    match solsta {
        MSK_SOL_STA_OPTIMAL => {
            let mut xx = vec![0.0; numvar as usize];
            task.get_xx(MSK_SOL_ITR,    /* Request the basic solution. */
                        & mut xx[..]);
            
            println!("Optimal primal solution");
            for j in 0..numvar as usize
            {
                println!("x[{}]: {}",j,xx[j]);
            }
        }
        MSK_SOL_STA_DUAL_INFEAS_CER |
        MSK_SOL_STA_PRIM_INFEAS_CER => {
            println!("Primal or dual infeasibility certificate found.");
        }

        MSK_SOL_STA_UNKNOWN => {
            /* If the solutions status is unknown, print the termination code
             * indicating why the optimizer terminated prematurely. */

            println!("The solution status is unknown.");
            println!("The optimizer terminitated with code: {}",solsta);
          }
        _ => {
            println!("Other solution status.");
        }
    }
}
