//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : acc1.rs
//!
//!  Purpose :   Tutorial example for affine conic constraints.
//!              Models the problem:
//!
//!              maximize c^T x
//!              subject to  sum(x) = 1
//!                          gamma >= |Gx+h|_2
//!

extern crate mosek;
extern crate itertools;
use mosek::{Task,Objsense,Streamtype,Solsta,Soltype,Boundkey};

// Define problem data
#[allow(non_upper_case_globals)]
const n : i32 = 3;
#[allow(non_upper_case_globals)]
const k : i64 = 2;

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
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
    task.put_var_bound_slice_const(0, n, Boundkey::FR, 0.0, 0.0)?;

    // Set up the objective
    let c = &[2.0, 3.0, -1.0];
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    task.put_c_list(x.as_slice(), c)?;

    // One linear constraint - sum(x) = 1
    task.append_cons(1)?;
    task.put_a_row(0,x.as_slice(), vec![1.0; n as usize].as_slice())?;
    task.put_con_bound(0, Boundkey::FX, 1.0, 1.0)?;

    // Append empty AFE rows for affine expression storage
    task.append_afes(k + 1)?;

    // G matrix in sparse form
    let Gsubi : &[i64] = &[0, 0, 1, 1];
    let Gsubj : &[i32] = &[0, 1, 0, 2];
    let Gval           = &[1.5, 0.1, 0.3, 2.1];
    // Other data
    let h     = &[0.0, 0.1];
    let gamma = 0.03;

    // Construct F matrix in sparse form
    let Fsubi : Vec<i64> = Gsubi.iter().map(|i| *i+1).collect(); // G will be placed from row number 1 in F
    let Fsubj = Gsubj;
    let Fval  = Gval;

    // Fill in F storage
    task.put_afe_f_entry_list(Fsubi.as_slice(), Fsubj, Fval)?;
    // Fill in g storage
    task.put_afe_g(0, gamma)?;
    task.put_afe_g_slice(1, k+1, h)?;

    // Define a conic quadratic domain
    let quadDom = task.append_quadratic_cone_domain(k + 1)?;

    // Create the ACC
    task.append_acc(quadDom,    // Domain index
                    (0..k+1).collect::<Vec<i64>>().as_slice(), // Indices of AFE rows [0,...,k]
                    vec![0.0; (k+1) as usize].as_slice())?;       // Ignored

    // Solve and retrieve solution
    let _ = task.optimize()?;
    task.write_data("acc1.ptf")?;
    let mut xx = vec![0.0; n as usize];
    task.get_xx(Soltype::ITR,xx.as_mut_slice())?;

    assert!(task.get_sol_sta(Soltype::ITR)? == Solsta::OPTIMAL);
    println!("Solution: {:?}",xx);

    // Demonstrate retrieving activity of ACC
    let mut activity = vec![0.0; (k+1) as usize];
    let mut doty     = vec![0.0; (k+1) as usize];

    task.evaluate_acc(Soltype::ITR,0,activity.as_mut_slice())?;
    println!("Activity of ACC:: {:?}",activity);

    // Demonstrate retrieving the dual of ACC
    task.get_acc_dot_y(Soltype::ITR,0,doty.as_mut_slice())?;
    println!("Dual of ACC:: {:?}",doty);


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
