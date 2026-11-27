//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//! 
//!   File : sdo_lmi.rs
//! 
//!   Purpose :   To solve a problem with an LMI and an affine conic constrained problem with a PSD term
//!    
//!                 minimize    Tr [1, 0; 0, 1]*X + x(1) + x(2) + 1
//!
//!                 subject to  Tr [0, 1; 1, 0]*X - x(1) - x(2) >= 0
//!                             x(1) [0, 1; 1, 3] + x(2) [3, 1; 1, 0] - [1, 0; 0, 1] >> 0
//!                             X >> 0
//!

extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype,Solsta,Soltype};

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    let numafe : i64 = 4;  /* Number of affine expressions.              */
    let numvar : i32 = 2;  /* Number of scalar variables */
    let dimbarvar = &[2];         /* Dimension of semidefinite cone */
    let lenbarvar = &[2 * (2 + 1) / 2]; /* Number of scalar SD variables  */

    let barc_j  = &[0, 0];
    let barc_k  = &[0, 1];
    let barc_l  = &[0, 1];
    let barc_v  = &[1.0, 1.0];

    let afeidx  = &[0, 0, 1, 2, 2, 3];
    let varidx  = &[0, 1, 1, 0, 1, 0];
    let f_val  = &[-1.0, -1.0, 3.0, 2.0f64.sqrt(), 2.0f64.sqrt(), 3.0];
    let g       = &[0.0, -1.0, 0.0, -1.0];

    let barf_i = &[0, 0];
    let barf_j = &[0, 0];
    let barf_k = &[0, 1];
    let barf_l = &[0, 0];
    let barf_v = &[0.0, 1.0];

    let mut task = Task::new().unwrap().with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Append 'NUMAFE' empty affine expressions. */
    task.append_afes(numafe)?;

    /* Append 'NUMVAR' variables.
    The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar)?;

    /* Append 'NUMBARVAR' semidefinite variables. */
    task.append_barvars(dimbarvar)?;


    task.put_obj_sense(Objsense::MINIMIZE)?;
    /* Optionally add a constant term to the objective. */
    task.put_cfix(1.0)?;

      /* Set the linear term c_j in the objective.*/
    task.put_c_j(0, 1.0)?;
    task.put_c_j(1, 1.0)?;

    task.put_var_bound_slice_const(0,numvar, Boundkey::FR, -INF,INF)?;

    /* Set the linear term barc_j in the objective.*/
    task.put_barc_block_triplet(barc_j, barc_k, barc_l, barc_v)?;

    /* Set up the affine conic constraints */

    /* Construct the affine expressions */
    /* F matrix */
    task.put_afe_f_entry_list(afeidx, varidx, f_val)?;
    /* g vector */
    task.put_afe_g_slice(0, 4, g)?;

    /* barF block triplets */
    task.put_afe_barf_block_triplet(barf_i, barf_j, barf_k, barf_l, barf_v)?;

    /* Append R+ domain and the corresponding ACC */
    {
        let dom = task.append_rplus_domain(1)?;
        task.append_acc(dom, &[0], &[0.0])?;
    }

    /* Append SVEC_PSD domain and the corresponding ACC */
    {
        let dom = task.append_svec_psd_cone_domain(3)?;
        task.append_acc(dom, &[1,2,3], &[0.0,0.0,0.0])?;
    }

    /* Run optimizer */
    let _ = task.optimize()?;

    /* Print a summary containing information
    about the solution for debugging purposes*/
    task.solution_summary (mosek::Streamtype::MSG)?;

    let solsta = task.get_sol_sta(mosek::Soltype::ITR)?;

    match solsta {
        Solsta::OPTIMAL => {
            let mut xx = vec![0.0; numvar as usize];
            task.get_xx(Soltype::ITR,xx.as_mut_slice())?;
            let mut barx = vec![0.0; lenbarvar[0]];
            task.get_barx_j(Soltype::ITR, 0, barx.as_mut_slice())?;    /* Request the interior solution. */
            println!("Optimal primal solution");
            println!("  x = {:?}",xx);
            println!("  barx = {:?}",barx);
        },
        Solsta::DUAL_INFEAS_CER|Solsta::PRIM_INFEAS_CER =>
          println!("Primal or dual infeasibility certificate found."),
        Solsta::UNKNOWN =>
          println!("The status of the solution could not be determined."),
        _ =>
          println!("Other solution status.")
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
