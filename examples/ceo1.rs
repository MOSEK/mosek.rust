/*
   Copyright : $$copyright

   File :      $${file}

   Purpose :   Demonstrates how to solve a small conic exponential
               optimization problem using the MOSEK API.
*/
/*TAG:begin-code*/
extern crate mosek;

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    let numcon = 1;
    let numvar = 3;

    let bkc = mosek.boundkey.fx;
    let blc = 1.0;
    let buc = 1.0;

    let bkx = vec![ mosek::MSK_BK_FR,
                    mosek::MSK_BK_FR,
                    mosek::MSK_BK_FR ];
    let blx = vec![ -INF, -INF, -INF ];
    let bux = vec![ +INF, +INF, +INF ];
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
    task.put_a_row(0, asub.as_slice(), a)?;
    task.put_con_bound(0, bkc, blc, buc)?;
    task.put_var_bound_slice(0, numvar, bkx.as_slice(), blx.as_slice(), bux.as_slice())?;

    /* Add a conic constraint */
    task.append_cone(mosek::MSK_CT_PEXP,
                     0.0, /* For future use only, can be set to 0.0 */
                     csub.as_slice())?;
    task.put_obj_sense(mosek::MSK_OBJ_SENSE_MINIMIZE)?;

    println!("optimize");
    /* Solve the problem */
    task.optimize()?;
    // Print a summary containing information
    // about the solution for debugging purposes
    task.solution_summary(mosek::MSK_STREAM_MSG);

    mosek.solsta solsta[] = new mosek.solsta[1];

    /* Get status information about the solution */
    let solsta = task.get_sol_sta(mosek::MSK_SOL_ITR);

    task.getxx(mosek.soltype.itr, // Interior solution.
                 xx);
      /*TAG:end-getsolution*/

      switch (solsta[0]) {
        case optimal:
          System.out.println("Optimal primal solution\n");
          for (int j = 0; j < numvar; ++j)
            System.out.println ("x[" + j + "]:" + xx[j]);
          break;
        case dual_infeas_cer:
        case prim_infeas_cer:
          System.out.println("Primal or dual infeasibility.\n");
          break;
        case unknown:
          System.out.println("Unknown solution status.\n");
          break;
        default:
          System.out.println("Other solution status");
          break;
      }
    } catch (mosek.Exception e) {
      System.out.println ("An error/warning was encountered");
      System.out.println (e.toString());
      throw e;
    }
  }
}
/*TAG:end-code*/
