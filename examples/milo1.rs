//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : milo1.rs
//!
//!   Purpose :   Demonstrates how to solve a small mixed
//!               integer linear optimization problem using the MOSEK Java API.
//!
extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Prosta,Soltype,Variabletype,Dparam};

fn main() -> Result<(),String> {
    let numcon : i32 = 2;
    let numvar : i32 = 2;

    let infinity = 0.0; // only for symbolic purposes, value never used

    let bkc = vec![Boundkey::UP, Boundkey::LO];
    let blc = vec![ -infinity,         -4.0 ];
    let buc = vec![ 250.0,             infinity ];

    let bkx = vec![ Boundkey::LO, Boundkey::LO  ];
    let blx = vec![ 0.0,               0.0 ];
    let bux = vec![ infinity,          infinity ];

    let c   = vec![1.0, 0.64];

    let asub = vec![0,   1,
                    0,    1];
    let aval = vec![50.0, 3.0, 31.0, -2.0];

    let ptrb : Vec<usize> = vec![ 0, 2 ];
    let ptre : Vec<usize> = vec![ 2, 4 ];

    /* Create the optimization task. */
    Task::new().expect("Failed to create task")
        .with_stream_callback(
            Streamtype::LOG,
            &mut |msg| print!("{}",msg),
            |task| task.with_itg_sol_callback(
                &mut |xx| { println!("Found a new solution = {:?}",xx); false },
                |task| {
                    /* Append 'numcon' empty constraints.
                    The constraints will initially have no bounds. */
                    task.append_cons(numcon)?;

                    /* Append 'numvar' variables.
                    The variables will initially be fixed at zero (x=0). */
                    task.append_vars(numvar)?;

                    for ((((j,cj),bk),bl),bu) in (0..numvar).zip(c.iter()).zip(bkx.iter()).zip(blx.iter()).zip(bux.iter()) {
                        /* Set the linear term c_j in the objective.*/
                        task.put_c_j(j, *cj)?;
                        /* Set the bounds on variable j.
                           blx[j] <= x_j <= bux[j] */
                        task.put_var_bound(j, *bk, *bl, *bu)?;
                        /* Input column j of A */
                        task.put_a_col(j,                     /* Variable (column) index.*/
                                       &asub[ptrb[j as usize]..ptre[j as usize]],               /* Row index of non-zeros in column j.*/
                                       &aval[ptrb[j as usize]..ptre[j as usize]])?;              /* Non-zero Values of column j. */
                    }
                    // Set the bounds on constraints.
                    // for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] 
                    for (((i,bk),bl),bu) in (0..numcon).zip(bkc.iter()).zip(blc.iter()).zip(buc.iter()) {
                        task.put_con_bound(i, *bk, *bl, *bu)?;
                    }

                    /* Specify integer variables. */
                    for j in 0..numvar {
                        task.put_var_type(j, Variabletype::TYPE_INT)?;
                    }
                    /* Set max solution time */
                    task.put_dou_param(Dparam::MIO_MAX_TIME, 60.0)?;

                    /* A maximization problem */
                    task.put_obj_sense(Objsense::MAXIMIZE)?;
                    /* Solve the problem */

                    let _trm = task.optimize()?;

                    // Print a summary containing information
                    //   about the solution for debugging purposes
                    task.solution_summary(Streamtype::MSG)?;

                    let mut xx = vec![0.0; numvar as usize];
                    task.get_xx(Soltype::ITG, xx.as_mut_slice())?;

                    /* Get status information about the solution */

                    match task.get_sol_sta(Soltype::ITG)? {
                        Solsta::INTEGER_OPTIMAL => {
                            println!("Optimal solution");
                            for (j,xj) in (0..numvar).zip(xx.iter()) {
                                println!("x[{}]: {}", j,xj);
                            }
                        }
                        Solsta::PRIM_FEAS => {
                            println!("Feasible solution");
                            for (j,xj) in (0..numvar).zip(xx.iter()) {
                                println!("x[{}]: {}", j,xj);
                            }
                        }
                        Solsta::UNKNOWN => {
                          match task.get_pro_sta(Soltype::ITG)? {
                              Prosta::PRIM_INFEAS_OR_UNBOUNDED => {
                                  println!("Problem status Infeasible or unbounded");
                              }
                              Prosta::PRIM_INFEAS => {
                                  println!("Problem status Infeasible.");
                              }
                              Prosta::UNKNOWN => {
                                  println!("Problem status unknown.");
                              }
                              _ => {
                                  println!("Other problem status.");
                              }
                          }
                        }
                        _ => {
                            println!("Other solution status");
                        }
                    }
                    Ok(())
                }))
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
