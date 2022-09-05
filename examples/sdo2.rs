//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : sdo2.rs
//!
//!  Purpose :   Solves the semidefinite problem with two symmetric variables:
//!
//!                 min   <C1,X1> + <C2,X2>
//!                 st.   <A1,X1> + <A2,X2> = b
//!                             (X2)_{1,2} <= k
//!
//!                 where X1, X2 are symmetric positive semidefinite,
//!
//!                 C1, C2, A1, A2 are assumed to be constant symmetric matrices,
//!                 and b, k are constants.
//!

extern crate mosek;

use mosek::{Task,Streamtype,Solsta,Soltype};


#[allow(non_snake_case)]
fn main() -> Result<(),String> {

    /* Input data */
    let  numcon : i32       = 2;              /* Number of constraints. */
    let  dimbarvar : &[i32] = &[3, 4];         /* Dimension of semidefinite variables */

    /* Objective coefficients concatenated */
    let Cj : &[i32] = &[ 0, 0, 1, 1, 1, 1 ];   /* Which symmetric variable (j) */
    let Ck : &[i32] = &[ 0, 2, 0, 1, 1, 2 ];   /* Which entry (k,l)->v */
    let Cl : &[i32] = &[ 0, 2, 0, 0, 1, 2 ];
    let Cv : &[f64] = &[ 1.0, 6.0, 1.0, -3.0, 2.0, 1.0 ];

    /* Equality constraints coefficients concatenated */
    let Ai : &[i32] = &[ 0, 0, 0, 0, 0, 0 ];   /* Which constraint (i = 0) */
    let Aj : &[i32] = &[ 0, 0, 0, 1, 1, 1 ];   /* Which symmetric variable (j) */
    let Ak : &[i32] = &[ 0, 2, 2, 1, 1, 3 ];   /* Which entry (k,l)->v */
    let Al : &[i32] = &[ 0, 0, 2, 0, 1, 3 ];
    let Av : &[f64] = &[ 1.0, 1.0, 2.0, 1.0, -1.0, -3.0 ];

    /* The second constraint - one-term inequality */
    let A2i : &[i32] = &[ 1 ];                        /* Which constraint (i = 1) */
    let A2j : &[i32] = &[ 1 ];                        /* Which symmetric variable (j = 1) */
    let A2k : &[i32] = &[ 1 ];                        /* Which entry A(1,0) = A(0,1) = 0.5 */
    let A2l : &[i32] = &[ 0 ];
    let A2v : &[f64] = &[ 0.5 ];

    let bkc = &[ mosek::Boundkey::FX,
                 mosek::Boundkey::UP ];
    let blc = &[ 23.0,  0.0 ];
    let buc = &[ 23.0, -3.0 ];

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    }.with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Append numcon empty constraints.
    The constraints will initially have no bounds. */
    task.append_cons(numcon)?;

    /* Append numbarvar semidefinite variables. */
    task.append_barvars(dimbarvar)?;

    /* Set objective (6 nonzeros).*/
    task.put_barc_block_triplet(Cj, Ck, Cl, Cv)?;

    /* Set the equality constraint (6 nonzeros).*/
    task.put_bara_block_triplet(Ai, Aj, Ak, Al, Av)?;

    /* Set the inequality constraint (1 nonzero).*/
    task.put_bara_block_triplet(A2i, A2j, A2k, A2l, A2v)?;

    /* Set constraint bounds */
    task.put_con_bound_slice(0, 2, bkc, blc, buc)?;

    /* Run optimizer */
    task.optimize()?;
    task.solution_summary(Streamtype::MSG)?;

    //mosek.solsta[] solsta = new mosek.solsta[1];
    let solsta = task.get_sol_sta (Soltype::ITR)?;

    match solsta {
        Solsta::OPTIMAL => {
            /* Retrieve the soution for all symmetric variables */
            println!("Solution (lower triangular part vectorized):");
            for (i,dimbarvari) in dimbarvar.iter().enumerate() {
                //let dim = dimbarvar[i] * (dimbarvar[i] + 1) / 2;
                let dim = dimbarvari * (dimbarvari+1)/2;
                //double[] barx = new double[dim];
                let mut barx : Vec<f64> = vec![0.0; dim as usize];
                task.get_barx_j(Soltype::ITR, i as i32, barx.as_mut_slice())?;

                println!("X{}: {:?}",i+1,barx);
                // for (int j = 0; j < dim; ++j)
                //     System.out.print(barx[j] + " ");
                // System.out.println();
            }
        },
        Solsta::DUAL_INFEAS_CER|Solsta::PRIM_INFEAS_CER =>
            println!("Primal or dual infeasibility certificate found."),
        Solsta::UNKNOWN =>
            println!("The status of the solution could not be determined."),
        _ => println!("Other solution status.")
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
