//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : acc2.rs
//!
//!  Purpose :   Tutorial example for affine conic constraints.
//!              Models the problem:
//!
//!              maximize c^T x
//!              subject to  sum(x) = 1
//!                          gamma >= |Gx+h|_2
//!
//!              This version inputs the linear constraint as an affine conic constraint.
//!
extern crate mosek;
extern crate itertools;
use mosek::{Task,Objsense,Streamtype,Solsta,Soltype,Boundkey};

#[allow(non_upper_case_globals)]
fn main() -> Result<(),String> {
    // Define problem data
    const n : i32 = 3;
    const k : i64 = 2;

    // Create a task
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    }.with_callbacks();
    // Attach a printer to the task
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
    // Create n free variables
    task.append_vars(n)?;
    let x : Vec<i32> = (0..n).collect();

    task.put_var_bound_slice_const(0, n, Boundkey::FR, 0.0,0.0)?;

    // Set up the objective
    let c = &[2.0, 3.0, -1.0];
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    task.put_c_list(x.as_slice(), c)?;

    // Set AFE rows representing the linear constraint
    task.append_afes(1)?;
    task.put_afe_f_row(0, x.as_slice(), vec![1.0; n as usize].as_slice())?;
    task.put_afe_g(0, -1.0)?;

    // Set AFE rows representing the quadratic constraint
    task.append_afes(k + 1)?;
    task.put_afe_f_row(2,             // afeidx, row number
                    &[0i32, 1],    // varidx, column numbers
                    &[1.5, 0.1])?; // values
    task.put_afe_f_row(3,          // afeidx, row number
                    &[0i32, 2],     // varidx, column numbers
                    &[0.3, 2.1])?; // values

    let h = &[0.0, 0.1];
    let gamma = 0.03;
    task.put_afe_g(1, gamma)?;
    task.put_afe_g_slice(2, k+2, h)?;

    // Define domains
    let zero_dom = task.append_rzero_domain(1)?;
    let quad_dom = task.append_quadratic_cone_domain(k + 1)?;

    // Append affine conic constraints
    task.append_acc(zero_dom,    // Domain index
                    &[0i64],        // Indices of AFE rows
                    &[0.0])?;       // Ignored
    task.append_acc(quad_dom,    // Domain index
                   &[1i64,2,3],    // Indices of AFE rows
                   &[0.0,0.0,0.0])?; // Ignored

    // Solve and retrieve solution
    let _ = task.optimize()?;
    task.write_data("acc2.ptf")?;
    let mut xx = vec![0.0; n as usize];
    task.get_xx(Soltype::ITR,xx.as_mut_slice())?;
    assert! (task.get_sol_sta(Soltype::ITR)? == Solsta::OPTIMAL);
    println!("Solution: {:?}",xx);

    // Demonstrate retrieving activity of ACC
    let mut activity = vec![0.0; 3];
    let mut doty     = vec![0.0; 3];
    task.evaluate_acc(Soltype::ITR,1,activity.as_mut_slice())?;
    println!("Activity of quadratic ACC:: {:?}",activity);

    // Demonstrate retrieving the dual of ACC
    task.get_acc_dot_y(Soltype::ITR,1,doty.as_mut_slice())?;
    println!("Dual of quadratic ACC:: {:?}",doty);


    Ok(())
}

fn maxgap(a : &[f64], b : &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&a,&b)| (a - b).abs()).max_by(|a,b| if a < b { std::cmp::Ordering::Less } else if b < a { std::cmp::Ordering::Greater } else {std::cmp::Ordering::Equal} ).unwrap()
}
fn dot(a : &[f64], b : &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&a,&b)| (a * b)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
