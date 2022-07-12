/*
   Copyright: MOSEK ApS

   Purpose:   To demonstrate how to solve a small conic quadratic
              optimization problem using the MOSEK API.
 */

extern crate mosek;

const INF : f64 = 0.0;

use mosek::{Task,Streamtype,Solsta,Soltype};

fn main() -> Result<(),String>
{
    let numvar  : usize = 6;
    let numcon  : usize = 1;

    let bkc = vec![ mosek::Boundkey::FX ];
    let blc = vec![ 1.0 ];
    let buc = vec![ 1.0 ];

    let bkx = vec![ mosek::Boundkey::LO,
                    mosek::Boundkey::LO,
                    mosek::Boundkey::LO,
                    mosek::Boundkey::FR,
                    mosek::Boundkey::FR,
                    mosek::Boundkey::FR];
    let blx = vec![ 0.0, 0.0, 0.0, -INF, -INF, -INF ];
    let bux = vec![ INF, INF, INF,  INF,  INF,  INF ];
    let c   = vec![ 0.0, 0.0, 0.0,  1.0,  1.0,  1.0 ];

    let aptrb = vec![0, 1, 2, 3, 3, 3];
    let aptre = vec![1, 2, 3, 3, 3, 3];
    let asub  = vec![0, 0, 0, 0];
    let aval  = vec![1.0, 1.0, 2.0];

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();

    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Append 'numcon' empty constraints.
     * The constraints will initially have no bounds. */
    task.append_cons(numcon as i32)?;

    /* Append 'numvar' variables.
     * The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar as i32)?;

    for j in 0..numvar
    {
        /* Set the linear term c_j in the objective.*/
        task.put_c_j(j as i32,c[j])?;

        /* Set the bounds on variable j.
         * blx[j] <= x_j <= bux[j] */
        task.put_var_bound( j as i32,    /* Index of variable.*/
                            bkx[j],      /* Bound key.*/
                            blx[j],      /* Numerical value of lower bound.*/
                            bux[j])?;     /* Numerical value of upper bound.*/

        /* Input column j of A */
        task.put_a_col( j as i32,                      /* Variable (column) index.*/
                        &asub[aptrb[j]..aptre[j]],     /* Pointer to row indexes of column j.*/
                        &aval[aptrb[j]..aptre[j]])?;    /* Pointer to Values of column j.*/

    }

    /* Set the bounds on constraints.
     * for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] */
    for i in 0..numcon
    {
        task.put_con_bound( i as i32,    /* Index of constraint.*/
                            bkc[i],      /* Bound key.*/
                            blc[i],      /* Numerical value of lower bound.*/
                            buc[i])?;     /* Numerical value of upper bound.*/
    }

    /* Append the first cone. */
//TAG:begin-appendcone
        // Create a matrix F such that F * x = [x(3),x(0),x(1),x(4),x(5),x(2)]
    {
        task.append_afes(6)?;
        task.put_afe_f_entry_list(&[0,1,2,3,4,5],                      // Rows
                                  &[3, 0, 1, 4, 5, 2],            // Columns
                                  &[1.0,1.0,1.0,1.0,1.0,1.0])?;

        // Quadratic cone (x(3),x(0),x(1)) \in QUAD_3
        let qconedom  = task.append_quadratic_cone_domain(3)?;
        task.append_acc(qconedom,                // Domain
                        &[0, 1, 2],              // Rows from F
                        &[0.0,0.0,0.0])?;        // Unused

        // Rotated quadratic cone (x(4),x(5),x(2)) \in RQUAD_3
        let rqconedom = task.append_r_quadratic_cone_domain(3)?;
        task.append_acc(rqconedom,               // Domain
                        &[3, 4, 5],              // Rows from F
                        &[0.0,0.0,0.0])?;        // Unused
    }
//TAG:end-appendcone

    /* Run optimizer */
    task.optimize()?;

    /* Print a summary containing information
     * about the solution for debugging purposes*/
    task.solution_summary (Streamtype::MSG)?;

    let solsta = task.get_sol_sta(Soltype::ITR)?;

    match solsta
    {
        Solsta::OPTIMAL =>
        {
            let mut xx = vec![0.0,0.0,0.0,0.0,0.0,0.0];
            task.get_xx(Soltype::ITR,    /* Request the basic solution. */
                        & mut xx[..])?;
            println!("Optimal primal solution");
            for j in 0..numvar as usize
            {
                println!("x[{}]: {}",j,xx[j]);
            }
          }

        Solsta::DUAL_INFEAS_CER |
        Solsta::PRIM_INFEAS_CER =>
        {
            println!("Primal or dual infeasibility certificate found.");
        }

        Solsta::UNKNOWN =>
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
    Ok(())
} /* main */
