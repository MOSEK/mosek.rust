/*
   Copyright
       $$copyright
   File
       $${file}
   Description
       Demonstrates how to solve a small conic exponential
       optimization problem using the MOSEK API.

       Min x1 + x2
       Such that
           x1 + x2 + x3 = 1.0
           |1    |   |x1|
           |  1  | x |x2| in K_exp
           |    1|   |x3|
           x1,x2,x3 are free
*/
extern crate mosek;

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    let numcon = 1;
    let numvar = 3;

    let bkc = mosek::MSK_BK_FX;
    let blc = 1.0;
    let buc = 1.0;

    let bkx = vec![ mosek::MSK_BK_FR,
                    mosek::MSK_BK_FR,
                    mosek::MSK_BK_FR ];
    let blx = vec![ -INF, -INF, -INF ];
    let bux = vec![ INF, INF, INF ];
    let c   = vec![ 1.0, 1.0, 0.0 ];
    let a   = vec![ 1.0, 1.0, 1.0 ];
    let asub = vec![0, 1, 2];
    let csub = vec![0, 1, 2];
    //let csub = new int[numvar];
    //double[] xx  = new double[numvar];

    /* Create the mosek environment. */
    let env = match mosek::Env::new() {
        Some(e) => e,
        None => return Err("Failed to create env".to_string()),
        };
    /* Create the optimization task. */
    let mut task = match env.task() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        };

    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;
    task.put_callback(|caller,_,_,_| { println!("caller = {}",caller); true })?;

    /* Append 'numcon' empty constraints.
       The constraints will initially have no bounds. */
    task.append_cons(numcon)?;

      /* Append 'numvar' variables.
         The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar)?;

    /* Define the linear part of the problem */
    task.put_c_slice(0, numvar, c.as_slice())?;
    task.put_a_row(0, asub.as_slice(), a.as_slice())?;
    task.put_con_bound(0, bkc, blc, buc)?;
    task.put_var_bound_slice(0, numvar, bkx.as_slice(), blx.as_slice(), bux.as_slice())?;

    /* Add a conic constraint */
    task.append_afes(3)?;
    let afeidxs = vec![0,  1,  2  ];
    let b       = vec![0.0,0.0,0.0];
    let domidx  = task.append_primal_exp_cone_domain()?;
    task.put_afe_f_row_list(afeidxs.as_slice(),
                            vec![1,1,1].as_slice(),
                            vec![0,1,2].as_slice(),
                            vec![0,1,2].as_slice(),
                            vec![1.0,1.0,1.0].as_slice())?;
    task.append_acc(domidx,afeidxs.as_slice(),b.as_slice())?;

    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MINIMIZE)?;

    println!("optimize");
    /* Solve the problem */
    task.optimize()?;
    // Print a summary containing information
    // about the solution for debugging purposes
    task.solution_summary(mosek::MSK_STREAM_MSG)?;

    /* Get status information about the solution */
    let solsta = task.get_sol_sta(mosek::MSK_SOL_ITR)?;

    match solsta {
        mosek::MSK_SOL_STA_OPTIMAL => {
            let mut xx = vec![0.0; numvar as usize];
            task.get_xx(mosek::MSK_SOL_ITR, & mut xx[..])?;
            println!("Optimal primal solution");
            for j in 0..numvar as usize {
                println!("x[{}]: {:.4}",j,xx[j]);
            }
          }

        mosek::MSK_SOL_STA_DUAL_INFEAS_CER |
        mosek::MSK_SOL_STA_PRIM_INFEAS_CER => {
            println!("Primal or dual infeasibility certificate found.");
        }

        mosek::MSK_SOL_STA_UNKNOWN => {
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
    return Result::Ok(());
}
