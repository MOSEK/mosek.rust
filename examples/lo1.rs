/*
  Copyright: MOSEK ApS

  Purpose:   To demonstrate how to solve a small linear
             optimization problem using the MOSEK C API, 
             and handle the solver result and the problem
             solution.
*/

extern crate mosek;

const INF : f64 = 0.0;

fn main()
{
    let numvar = 4;
    let numcon = 3;

    let c = vec![3.0, 1.0, 5.0, 1.0];

    /* Below is the sparse representation of the A
     * matrix stored by column. */
    let aptrb = vec![ 0, 2, 5, 7 ];
    let aptre = vec![ 2, 5, 7, 9 ];
    let asub  = vec![ 0, 1,
                      0, 1, 2,
                      0, 1,
                      1, 2 ];
    let aval  = vec![ 3.0, 2.0,
                      1.0, 1.0, 2.0,
                      2.0, 3.0,
                      1.0, 3.0 ];

    /* Bounds on constraints. */
    let bkc = vec![ mosek::MSK_BK_FX, mosek::MSK_BK_LO, mosek::MSK_BK_UP ];
    let blc = vec![ 30.0,      15.0,      -INF      ];
    let buc = vec![ 30.0,      INF,       25.0      ];
    /* Bounds on variables. */
    let bkx = vec![ mosek::MSK_BK_LO, mosek::MSK_BK_RA, mosek::MSK_BK_LO, mosek::MSK_BK_LO ];
    let blx = vec![ 0.0,       0.0,       0.0,       0.0       ];
    let bux = vec![ INF,      10.0,       INF,       INF       ];

    /* Create the mosek environment. */
    let env = mosek::Env::new();
    /* Create the optimization task. */
    let mut task = env.task();

    //task.put_stream_callback(mosek::MSK_STREAM_LOG, stream_func);
    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg));
    task.put_callback(|caller,_,_,_| { println!("caller = {}",caller); true } );

    /* Directs the log task stream to the 'printstr' function. */
    //task.linkfunctotaskstream(task,MSK_STREAM_LOG,NULL,printstr);

    /* Append 'numcon' empty constraints.
     * The constraints will initially have no bounds. */
    task.append_cons(numcon as i32);

    /* Append 'numvar' variables.
     * The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar as i32);

    for j in 0..numvar
    {
        /* Set the linear term c_j in the objective.*/
        task.put_c_j(j as i32,c[j]);

        /* Set the bounds on variable j.
         * blx[j] <= x_j <= bux[j] */
        task.put_var_bound(j as i32,    /* Index of variable.*/
                           bkx[j],      /* Bound key.*/
                           blx[j],      /* Numerical value of lower bound.*/
                           bux[j]);     /* Numerical value of upper bound.*/

        /* Input column j of A */
        task.put_a_col(j as i32,          /* Variable (column) index.*/
                       & asub[aptrb[j]..aptre[j]],     /* Pointer to row indexes of column j.*/
                       & aval[aptrb[j]..aptre[j]]);    /* Pointer to Values of column j.*/
    }

    /* Set the bounds on constraints.
     * for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] */
    for i in 0..numcon
    {
      task.put_con_bound(i as i32,    /* Index of constraint.*/
                         bkc[i],      /* Bound key.*/
                         blc[i],      /* Numerical value of lower bound.*/
                         buc[i]);     /* Numerical value of upper bound.*/
    }

    /* Maximize objective function. */
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE);

      /* Run optimizer */
    let trmcode = task.optimize();

    /* Print a summary containing information
     * about the solution for debugging purposes. */

    task.solution_summary(mosek::MSK_STREAM_LOG);

    let solsta = task.get_sol_sta(mosek::MSK_SOL_BAS);

    match solsta
    {
        mosek::MSK_SOL_STA_OPTIMAL =>
        {
            let mut xx = vec![0.0,0.0,0.0,0.0];
            task.get_xx(mosek::MSK_SOL_BAS,    /* Request the basic solution. */
                        & mut xx[..]);
            println!("Optimal primal solution");
            for j in 0..numvar as usize
            {
                println!("x[{}]: {}",j,xx[j]);
            }
          }

        mosek::MSK_SOL_STA_DUAL_INFEAS_CER |
        mosek::MSK_SOL_STA_PRIM_INFEAS_CER => 
        {
            println!("Primal or dual infeasibility certificate found.");
        }

        mosek::MSK_SOL_STA_UNKNOWN =>
        {
            /* If the solutions status is unknown, print the termination code
             * indicating why the optimizer terminated prematurely. */

            println!("The solution status is unknown.");
            println!("The optimizer terminitated with code: {}",solsta);
          }
        _ =>
        {
            println!("Other solution status.");
        }
    }
}
