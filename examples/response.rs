//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : response.rs
//!
//!  Purpose :   This example demonstrates proper response handling
//!              for problems solved with the interior-point optimizers.
//!
extern crate mosek;

use mosek::{Task,Streamtype,Solsta,Soltype};
use std::env;

const CQO1_PTF : &str = "Task 'CQO1 EXAMPLE'
Objective obj
    Minimize + x4 + x5 + x6
Constraints
    c1 [1] + x1 + x2 + 2 x3
Variables
    k1 [QUAD(3)]
        x4
        x1 [0;+inf]
        x2 [0;+inf]
    k2 [RQUAD(3)]
        x5
        x6
        x3 [0;+inf]
";


fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    let mut task = Task::new().unwrap().with_callbacks();
    if args.len() < 2 {
        task.read_ptf_string(CQO1_PTF)?;
    }
    else {
        task.read_data(args[1].as_str())?;
    }

    // Perform optimization.
    let trm = task.optimize()?;
    task.solution_summary(Streamtype::LOG)?;

    // Handle solution status. We expect Optimal
    let solsta = task.get_sol_sta(Soltype::ITR)?;

    match solsta {
        Solsta::OPTIMAL => {
            // Fetch and print the solution
            println!("An optimal interior point solution is located.");
            let numvar = task.get_num_var()?;
            let mut xx = vec![0.0; numvar as usize];
            task.get_xx(Soltype::ITR, xx.as_mut_slice())?;
            println!("xx = {:?}",xx)
        },
        Solsta::DUAL_INFEAS_CER =>
          println!("Dual infeasibility certificate found."),
        Solsta::PRIM_INFEAS_CER =>
          println!("Primal infeasibility certificate found."),
        Solsta::UNKNOWN => {
          // The solutions status is unknown. The termination code
          // indicates why the optimizer terminated prematurely.
          println!("The solution status is unknown.");
          let (symname,desc) = mosek::get_code_desc(trm)?;
          println!("   Termination code: {} {}\n", symname, desc)
        },
        _ =>
          println!("Unexpected solution status {}\n",solsta)
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
