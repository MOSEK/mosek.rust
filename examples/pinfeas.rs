//!
//!  File : pinfeas.rs
//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  Purpose: Demonstrates how to fetch a primal infeasibility certificate
//!           for a linear problem
//!

extern crate mosek;
use mosek::{Streamtype,Boundkey,Soltype,Prosta};

const INF : f64 = 0.0;

fn test_problem() -> Result<mosek::Task,String> {
    let mut task = mosek::Task::new().unwrap();
    task.append_vars(7)?;
    task.append_cons(7)?;
    task.put_c_list(&[0,1,2,3,4,5,6],
                    &[1.0,2.0,5.0,2.0,1.0,2.0,1.0])?;
    task.put_aij_list(&[0,0,1,1,2,2,2,3,3,4,5,5,6,6],
                      &[0,1,2,3,4,5,6,0,4,1,2,5,3,6],
                      &[1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0,1.0])?;
    task.put_con_bound_slice(0, 7,
                             &[Boundkey::UP,Boundkey::UP,Boundkey::UP,Boundkey::FX,Boundkey::FX,Boundkey::FX,Boundkey::FX],
                             &[-INF, -INF, -INF, 1100.0, 200.0, 500.0, 500.0],
                             &[200.0, 1000.0, 1000.0, 1100.0, 200.0, 500.0, 500.0])?;
    task.put_var_bound_slice_const(0, 7, Boundkey::UP, 0.0, INF)?;
    Ok(task)
}

// Analyzes and prints infeasibility contributing elements
// sl - dual values for lower bounds
// su - dual values for upper bounds
// eps - tolerance for when a nunzero dual value is significant
fn analyze_certificate(sl : &[f64], su : &[f64], eps : f64) {
    for (i,(&sli,&sui)) in sl.iter().zip(su.iter()).enumerate() {
        if sli > eps {
            println!("#{}, lower,  dual = {:e}\n", i, sli);
        }
        if sui > eps {
            println!("#{}, upper,  dual = {:e}\n", i, sui);
        }
    }
}

fn main() -> Result<(),String> {
    // In this example we set up a simple problem
    // One could use any task or a task read from a file
    let mut task = test_problem()?.with_callbacks();

    let n = task.get_num_var()?;
    let m = task.get_num_con()?;

    // Useful for debugging
    task.write_data("pinfeas.ptf")?; // Write file in human-readable format
    // Attach a log stream printer to the task
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // Perform the optimization.
    task.optimize()?;
    task.solution_summary(Streamtype::LOG)?;

    // Check problem status, we use the interior point solution
    if task.get_pro_sta(Soltype::ITR)? == Prosta::PRIM_INFEAS {
        // Set the tolerance at which we consider a dual value as essential
        let eps = 1e-7;

        println!("Variable bounds important for infeasibility: ");
        let mut slx = vec![0.0; n as usize]; task.get_slx(Soltype::ITR, slx.as_mut_slice())?;
        let mut sux = vec![0.0; n as usize]; task.get_sux(Soltype::ITR, sux.as_mut_slice())?;
        analyze_certificate(slx.as_slice(), sux.as_slice(), eps);

        println!("Constraint bounds important for infeasibility: ");
        let mut slc = vec![0.0; m as usize]; task.get_slc(Soltype::ITR, slc.as_mut_slice())?;
        let mut suc = vec![0.0; m as usize]; task.get_suc(Soltype::ITR, suc.as_mut_slice())?;
        analyze_certificate(slc.as_mut_slice(), suc.as_mut_slice(), eps);
    }
    else {
        println!("The problem is not primal infeasible, no certificate to show");
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
