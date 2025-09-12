//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : sdo1.rs
//!
//!  Purpose:   Solves the following small semidefinite optimization problem
//!             using the MOSEK API.
//!
//!    minimize    Tr [2, 1, 0; 1, 2, 1; 0, 1, 2]*X + x0
//!
//!    subject to  Tr [1, 0, 0; 0, 1, 0; 0, 0, 1]*X + x0           = 1
//!                Tr [1, 1, 1; 1, 1, 1; 1, 1, 1]*X      + x1 + x2 = 0.5
//!                (x0,x1,x2) \in Q,  X \in PSD
//!

extern crate mosek;

use mosek::{Task,Streamtype,Solsta,Soltype};

const INF : f64 = 0.0;

const NUMCON    : usize = 2;   /* Number of constraints.              */
const NUMVAR    : usize = 3;   /* Number of conic quadratic variables */

fn main() -> Result<(),String>
{
    let dimbarvar = vec![3];         /* Dimension of semidefinite cone */

    let bkc = &[ mosek::Boundkey::FX, mosek::Boundkey::FX ];
    let blc = &[ 1.0, 0.5 ];
    let buc = &[ 1.0, 0.5 ];

    let barc_i = &[0, 1, 1, 2, 2];
    let barc_j = &[0, 0, 1, 1, 2];
    let barc_v = &[2.0, 1.0, 2.0, 1.0, 2.0];

    let aptrb = &[0, 1];
    let aptre = &[1, 3];
    let asub  = &[0, 1, 2]; /* column subscripts of A */
    let aval  = &[1.0, 1.0, 1.0];

    let bara_i = &[0, 1, 2, 0, 1, 2, 1, 2, 2];
    let bara_j = &[0, 1, 2, 0, 0, 0, 1, 1, 2];
    let bara_v = &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let falpha = 1.0;


    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();

    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Append 'NUMCON' empty constraints.
     * The constraints will initially have no bounds. */
    task.append_cons(NUMCON as i32)?;

    /* Append 'NUMVAR' variables.
     * The variables will initially be fixed at zero (x=0). */
    task.append_vars(NUMVAR as i32)?;

    /* Append 'NUMBARVAR' semidefinite variables. */
    task.append_barvars(&dimbarvar[..])?;

    /* Optionally add a constant term to the objective. */
    task.put_cfix(0.0)?;

    /* Set the linear term c_j in the objective.*/
    task.put_c_j(0,1.0)?;

    for j in 0..NUMVAR {
        task.put_var_bound(j as i32,
                           mosek::Boundkey::FR,
                           -INF,
                           INF)?;
    }

    /* Set the linear term barc_j in the objective.*/
    let c_symmat_idx = task.append_sparse_sym_mat(dimbarvar[0],
                                                  barc_i,
                                                  barc_j,
                                                  barc_v)?;
    task.put_barc_j(0, &[c_symmat_idx], &[falpha])?;

    for i in 0..NUMCON
    {
        /* Input A row by row */
        task.put_a_row(i as i32,
                       & asub[aptrb[i]..aptre[i]],
                       & aval[aptrb[i]..aptre[i]])?;

        /* Set the bounds on constraints.
         * for i=1, ...,NUMCON : blc[i] <= constraint i <= buc[i] */
        task.put_con_bound(i as i32,    /* Index of constraint.*/
                           bkc[i],      /* Bound key.*/
                           blc[i],      /* Numerical value of lower bound.*/
                           buc[i])?;     /* Numerical value of upper bound.*/
    }

    {
        /* Append the conic quadratic cone */
        let afei = task.get_num_afe()?;
        task.append_afes(3)?;
        task.put_afe_f_entry_list(&[0,1,2],
                                  &[0,1,2],
                                  &[1.0,1.0,1.0])?;
        let dom = task.append_quadratic_cone_domain(3)?;
        task.append_acc_seq(dom,afei,&[0.0,0.0,0.0])?;
    }

    /* Add the first row of barA */
    let a_symmat_idx1 =
        task.append_sparse_sym_mat(dimbarvar[0],
                                   & bara_i[..3],
                                   & bara_j[..3],
                                   & bara_v[..3])?;

    task.put_bara_ij(0, 0, &[a_symmat_idx1][..], &[falpha][..])?;

    /* Add the second row of barA */
    let a_symmat_idx2 =
        task.append_sparse_sym_mat(dimbarvar[0],
                                   & bara_i[3..9],
                                   & bara_j[3..9],
                                   & bara_v[3..9])?;
    task.put_bara_ij(1, 0, &[a_symmat_idx2][..], &[falpha][..])?;

    let _trmcode = task.optimize()?;

    task.write_data("sdo1.ptf")?;
    /* Print a summary containing information
     * about the solution for debugging purposes*/
    task.solution_summary (Streamtype::MSG)?;

    let solsta = task.get_sol_sta(Soltype::ITR)?;

    match solsta
    {
        Solsta::OPTIMAL =>
        {
            let mut xx = vec![0.0,0.0,0.0];
            task.get_xx(Soltype::ITR,    /* Request the basic solution. */
                        & mut xx[..])?;
            let mut barx = vec![0.0,0.0,0.0,0.0,0.0,0.0];
            task.get_barx_j(Soltype::ITR,    /* Request the interior solution. */
                            0,
                            & mut barx[..])?;
            println!("Optimal primal solution");
            for j in 0..NUMVAR as usize
            {
                println!("x[{}]: {}",j,xx[j]);
            }
            let n = dimbarvar[0] as usize;
            for j in 0..n
            {
                for i in j..n
                {
                    println!("barx[{},{}]: {}",i,j,barx[j*n+i-j*(j+1)/2]);
                }
            }
          }

        Solsta::DUAL_INFEAS_CER       |
        Solsta::PRIM_INFEAS_CER       =>
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


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
