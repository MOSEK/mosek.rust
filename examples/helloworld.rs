//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : helloworld.rs
//!
//!  The most basic example of how to get started with MOSEK.
//!

extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Soltype};

fn main() -> Result<(),String> {
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        };

    task.append_vars(1)?;                           // 1 variable x
    task.put_c_j(0, 1.0)?;                          // c_0 = 1.0
    task.put_var_bound(0, Boundkey::RA, 2.0, 3.0)?; // 2.0 <= x <= 3.0
    task.put_obj_sense(Objsense::MINIMIZE)?;        // minimize

    task.optimize()?;                               // Optimize

    let mut x = vec![0.0; 1];
    task.get_xx(Soltype::ITR, x.as_mut_slice())?;   // Get solution
    println!("Solution x = {}", x[0]);              // Print solution
    return Result::Ok(());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
