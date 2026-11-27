//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : cqo1.rs
//!
//!  Purpose:   To demonstrate how to solve a small conic quadratic
//!             optimization problem using the MOSEK API.
//!
extern crate mosek;
extern crate itertools;

const INF : f64 = 0.0;

use mosek::{Task,Streamtype,Solsta,Soltype};
use itertools::izip;

fn main() -> Result<(),String>
{
    let numvar  : i32 = 6;
    let numcon  : i32 = 1;

    let bkc = &[ mosek::Boundkey::FX ];
    let blc = &[ 1.0 ];
    let buc = &[ 1.0 ];

    let bkx = &[ mosek::Boundkey::LO,
                    mosek::Boundkey::LO,
                    mosek::Boundkey::LO,
                    mosek::Boundkey::FR,
                    mosek::Boundkey::FR,
                    mosek::Boundkey::FR];
    let blx = &[ 0.0, 0.0, 0.0, -INF, -INF, -INF ];
    let bux = &[ INF, INF, INF,  INF,  INF,  INF ];
    let c   = &[ 0.0, 0.0, 0.0,  1.0,  1.0,  1.0 ];

    let asub  = &[0, 1, 2];
    let aval  = &[1.0, 1.0, 1.0];

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();

    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Append 'numcon' empty constraints.
     * The constraints will initially have no bounds. */
    task.append_cons(numcon)?;

    /* Append 'numvar' variables.
     * The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar)?;

    for (j,&cj,&bkj,&blj,&buj) in izip!(0..numvar,c,bkx,blx,bux) {
        /* Set the linear term c_j in the objective.*/
        task.put_c_j(j,cj)?;

        /* Set the bounds on variable j.
         * blx[j] <= x_j <= bux[j] */
        task.put_var_bound( j,    /* Index of variable.*/
                            bkj,      /* Bound key.*/
                            blj,      /* Numerical value of lower bound.*/
                            buj)?;     /* Numerical value of upper bound.*/
    }

    /* Input columns of A */
    task.put_a_row(0, asub, aval)?;

    /* Set the bounds on constraints.
     * for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] */
    for (i,&bki,&bli,&bui) in izip!(0..numcon,bkc,blc,buc) {
        task.put_con_bound( i,    /* Index of constraint.*/
                            bki,      /* Bound key.*/
                            bli,      /* Numerical value of lower bound.*/
                            bui)?;     /* Numerical value of upper bound.*/
    }


    /* Append the first cone. */
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

    /* Run optimizer */
    let trm = task.optimize()?;

    task.write_data("cqo1.ptf")?;
    /* Print a summary containing information
     * about the solution for debugging purposes*/
    task.solution_summary (Streamtype::MSG)?;

    let solsta = task.get_sol_sta(Soltype::ITR)?;

    match solsta
    {
        Solsta::OPTIMAL =>
        {
            let mut xx = vec![0.0; numvar as usize];
            task.get_xx(Soltype::ITR,    /* Request the basic solution. */
                        xx.as_mut_slice())?;
            println!("Optimal primal solution");
            for (j,xj) in izip!(0..numvar,xx) {
                println!("x[{}]: {}",j,xj);
            }
          }

        Solsta::DUAL_INFEAS_CER |
        Solsta::PRIM_INFEAS_CER =>
        {
            println!("Primal or dual infeasibility certificate found.");
        }

        Solsta::UNKNOWN => {
            /* If the solutions status is unknown, print the termination code
             * indicating why the optimizer terminated prematurely. */

            println!("The solution status is unknown.");
            println!("The optimizer terminitated with code: {}",trm);
          }
        _ =>
        {
            println!("Other solution status.");
        }
    }
    Ok(())
} /* main */

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
