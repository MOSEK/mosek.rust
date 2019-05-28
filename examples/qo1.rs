/*
   Copyright: MOSEK ApS

   Purpose: To demonstrate how to solve a quadratic optimization
              problem using the MOSEK API.
 */

extern crate mosek;

const INF : f64 = 0.0;


const NUMCON : usize = 1;   /* Number of constraints.             */
const NUMVAR : usize = 3;   /* Number of variables.               */

fn main()
{
    let c = vec![ 0.0,-1.0,0.0 ];

    let bkc = vec![ mosek::MSK_BK_LO ];
    let blc = vec![ 1.0 ];
    let buc = vec![ INF ];

    let bkx = vec![ mosek::MSK_BK_LO,
                    mosek::MSK_BK_LO,
                    mosek::MSK_BK_LO ];
    let blx = vec![ 0.0,
                    0.0,
                    0.0 ];
    let bux = vec![ INF,
                    INF,
                    INF ];

    let aptrb = vec![ 0,   1,   2 ];
    let aptre = vec![ 1,   2,   3 ];
    let asub  = vec![ 0,   0,   0 ];
    let aval  = vec![ 1.0, 1.0, 1.0 ];

    let qsubi = vec![ 0,  1,   2,  2 ];
    let qsubj = vec![ 0,  1,   0,  2 ];
    let qval  = vec![ 2.0,0.2,-1.0,2.0 ];


    /* Create the mosek environment. */
    let env = mosek::Env::new();
    /* Create the optimization task. */
    let mut task = env.task();

    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg));

    //r = MSK_linkfunctotaskstream(task,MSK_STREAM_LOG,NULL,printstr);

    task.append_cons(NUMCON as i32);

    /* Append 'NUMVAR' variables.
     * The variables will initially be fixed at zero (x=0). */
    task.append_vars(NUMVAR as i32);

    /* Optionally add a constant term to the objective. */
    task.put_cfix(0.0);

    for j in 0..NUMVAR 
    {
        /* Set the linear term c_j in the objective.*/
        task.put_c_j(j as i32,c[j]);

        /* Set the bounds on variable j.
         * blx[j] <= x_j <= bux[j] */
        task.put_var_bound(j as i32, /* Index of variable.*/
                           bkx[j],   /* Bound key.*/
                           blx[j],   /* Numerical value of lower bound.*/
                           bux[j]);  /* Numerical value of upper bound.*/
        /* Input column j of A */
        task.put_a_col(j as i32,                  /* Variable (column) index.*/
                       &asub[aptrb[j]..aptre[j]],  /* Pointer to row indexes of column j.*/
                       &aval[aptrb[j]..aptre[j]]); /* Pointer to Values of column j.*/
    }
    /* Set the bounds on constraints.
     * for i=1, ...,NUMCON : blc[i] <= constraint i <= buc[i] */
    for i in 0..NUMCON
    {
        task.put_con_bound(i as i32,    /* Index of constraint.*/
                           bkc[i],      /* Bound key.*/
                           blc[i],      /* Numerical value of lower bound.*/
                           buc[i]);     /* Numerical value of upper bound.*/

        /*
         * The lower triangular part of the Q
         * matrix in the objective is specified.
         */

        /* Input the Q for the objective. */

        task.put_q_obj(&qsubi,&qsubj,&qval);
    }


    let _trmcode = task.optimize();

    /* Run optimizer */
    /* Print a summary containing information
    about the solution for debugging purposes*/
    task.solution_summary (mosek::MSK_STREAM_MSG);

    let solsta = task.get_sol_sta(mosek::MSK_SOL_ITR);

    match solsta
    {
        mosek::MSK_SOL_STA_OPTIMAL =>
        {
            let mut xx = vec![0.0, 0.0, 0.0];
            task.get_xx(mosek::MSK_SOL_ITR,    /* Request the interior solution. */
                        & mut xx[..]);

            println!("Optimal primal solution");
            for j in 0..NUMVAR
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
            println!("The status of the solution could not be determined.");
        }

        _ =>
        {
            println!("Other solution status.");
        }
    }
} /* main */
