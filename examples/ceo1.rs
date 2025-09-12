//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : ceo1.rs
//!
//!   Description:
//!       Demonstrates how to solve a small conic exponential
//!       optimization problem using the MOSEK API.
//!
//!       Min x1 + x2
//!       Such that
//!           x1 + x2 + x3 = 1.0
//!           |1    |   |x1|
//!           |  1  | x |x2| in K_exp
//!           |    1|   |x3|
//!           x1,x2,x3 are free
//!
extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Soltype};

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    let numcon = 1;
    let numvar = 3;

    let bkc = mosek::Boundkey::FX;
    let blc = 1.0;
    let buc = 1.0;

    let bkx = vec![ Boundkey::FR,
                    Boundkey::FR,
                    Boundkey::FR ];
    let blx = vec![ -INF, -INF, -INF ];
    let bux = vec![ INF, INF, INF ];
    let c   = vec![ 1.0, 1.0, 0.0 ];
    let a   = vec![ 1.0, 1.0, 1.0 ];
    let asub = vec![0, 1, 2];

    /* Create the optimization task. */
    Task::new().expect("Failed to create task")
        .with_stream_callback(
            Streamtype::LOG, 
            &mut|msg| print!("{}",msg),
            |task| task.with_callback(
                &mut |caller| { println!("caller = {}",caller); false },
                |task| {
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

                    task.put_obj_sense(Objsense::MINIMIZE)?;

                    println!("optimize");
                    /* Solve the problem */
                    task.optimize()?;
                    // Print a summary containing information
                    // about the solution for debugging purposes
                    task.solution_summary(Streamtype::MSG)?;

                    /* Get status information about the solution */
                    let solsta = task.get_sol_sta(Soltype::ITR)?;

                    assert!(solsta == Solsta::OPTIMAL);
                    
                    let mut xx = vec![0.0; numvar as usize];
                    task.get_xx(Soltype::ITR, & mut xx[..])?;
                    
                    println!("Optimal primal solution");
                    for j in 0..numvar as usize {
                        println!("x[{}]: {:.4}",j,xx[j]);
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
