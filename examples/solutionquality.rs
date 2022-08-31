//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : solutionquality.rs
//!
//!  Purpose :   To demonstrate how to examine the quality of a solution.

/*TAG:begin-code*/
extern crate mosek;

use mosek::{Task,Streamtype,Solsta,Soltype};
use std::env;

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Syntax: solutionquality FILENAME");
        Err("Invalid argument list".to_string())
    } else {
        let mut task = Task::new().unwrap().with_callbacks();
        task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
        // We assume that a problem file was given as the first command
        // line argument (received in `args')
        task.read_data (args[0].as_str())?;

        // Solve the problem
        let _ = task.optimize()?;

        task.solution_summary(Streamtype::LOG)?;

        let solsta = task.get_sol_sta(Soltype::BAS)?;

        let mut pobj        : f64 = 0.0;
        let mut pviolcon    : f64 = 0.0;
        let mut pviolvar    : f64 = 0.0;
        let mut pviolbarvar : f64 = 0.0;
        let mut pviolcones  : f64 = 0.0;
        let mut pviolitg    : f64 = 0.0;
        let mut dobj        : f64 = 0.0;
        let mut dviolcon    : f64 = 0.0;
        let mut dviolvar    : f64 = 0.0;
        let mut dviolbarvar : f64 = 0.0;
        let mut dviolcones  : f64 = 0.0;

        task.get_solution_info(Soltype::BAS,
                               & mut pobj, & mut pviolcon, & mut pviolvar, & mut pviolbarvar, & mut pviolcones, & mut pviolitg,
                               & mut dobj, & mut dviolcon, & mut dviolvar, & mut dviolbarvar, & mut dviolcones)?;

        match solsta {  
            Solsta::OPTIMAL => {
                let abs_obj_gap = (dobj-pobj).abs();
                let rel_obj_gap = abs_obj_gap / (1.0 + f64::min(pobj.abs(), dobj.abs()));
                let max_primal_viol = f64::max(pviolcon, pviolvar);
                let max_primal_viol = f64::max(max_primal_viol  , pviolbarvar);
                let max_primal_viol = f64::max(max_primal_viol  , pviolcones);

                let max_dual_viol   = f64::max(dviolcon, dviolvar);
                let max_dual_viol   = f64::max(max_dual_viol, dviolbarvar);
                let max_dual_viol   = f64::max(max_dual_viol, dviolcones);

                // Assume the application needs the solution to be within
                //    1e-6 ofoptimality in an absolute sense. Another approach
                //   would be looking at the relative objective gap

                println!("Customized solution information.");
                println!("  Absolute objective gap: {:.3e}", abs_obj_gap);
                println!("  Relative objective gap: {:.3e}", rel_obj_gap);
                println!("  Max primal violation  : {:.3e}", max_primal_viol);
                println!("  Max dual violation    : {:.3e}", max_dual_viol);

                let mut accepted = true;

                if rel_obj_gap > 1e-6 {
                    println!("Warning: The relative objective gap is LARGE.");
                    accepted = false;
                }

                // We will accept a primal infeasibility of 1e-8 and
                // dual infeasibility of 1e-6. These number should chosen problem
                // dependent.
                if max_primal_viol > 1e-8 {
                    println!("Warning: Primal violation is too LARGE");
                    accepted = false;
                }

                if max_dual_viol > 1e-6 {
                    println!("Warning: Dual violation is too LARGE.");
                    accepted = false;
                }

                if accepted {
                    let numvar = task.get_num_var()?;
                    println!("Optimal primal solution");
                    let mut xx = vec![0.0; numvar as usize];
                    task.get_xx(Soltype::BAS,xx.as_mut_slice())?;
                    for (j,&xj) in (0..numvar).zip(xx.iter()) {
                        println!("x[{}]: {}",j,xj);
                    }
                } else {
                    // print etailed information about the solution
                    task.analyze_solution(Streamtype::LOG, Soltype::BAS)?;
                }
            },
            Solsta::DUAL_INFEAS_CER => println!("Primal or dual infeasibility certificate found."),
            Solsta::UNKNOWN => println!("The status of the solution is unknown."),
            _ => println!("Other solution status"),
        }
        Ok(())
    }
}
/*TAG:end-code*/
