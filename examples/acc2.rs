//
//  Copyright : MOSEK ApS
//
//  File :      acc2.rs
//
//  Purpose :   Tutorial example for affine conic constraints.
//              Models the problem:
//
//              maximize c^T x
//              subject to  sum(x) = 1
//                          gamma >= |Gx+h|_2
//
//              This version inputs the linear constraint as an affine conic constraint.
//
//TAG:begin-code

extern crate mosek;
extern crate itertools;
use mosek::{Task,Objsense,Streamtype,Solsta,Soltype,Boundkey};

fn main() -> Result<(),String> {
    // Define problem data
    const n : i32 = 3;
    const k : i64 = 2;

    // Create a task
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };
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

    //TAG:begin-putafe
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
    //TAG:end-putafe

    //TAG:begin-appenddomain
    // Define domains
    let zero_dom = task.append_rzero_domain(1)?;
    let quad_dom = task.append_quadratic_cone_domain(k + 1)?;
    //TAG:end-appenddomain

    // Append affine conic constraints
    //TAG:begin-appendacc1
    task.append_acc(zero_dom,    // Domain index
                    &[0i64],        // Indices of AFE rows
                    &[0.0])?;       // Ignored
    //TAG:end-appendacc1
    //TAG:begin-appendacc2
    task.append_acc(quad_dom,    // Domain index
                   &[1i64,2,3],    // Indices of AFE rows
                   &[0.0,0.0,0.0])?; // Ignored
    //TAG:end-appendacc2

    //TAG:begin-solve
    // Solve and retrieve solution
    let _ = task.optimize()?;
    task.write_data("acc2.ptf")?;
    let mut xx = vec![0.0; n as usize];
    task.get_xx(Soltype::ITR,xx.as_mut_slice())?;
    assert! (task.get_sol_sta(Soltype::ITR)? == Solsta::OPTIMAL);
    println!("Solution: {:?}",xx);
    //TAG:end-solve
    //TAG:end-code

    // Demonstrate retrieving activity of ACC
    let mut activity = vec![0.0; 3];
    let mut doty     = vec![0.0; 3];
    task.evaluate_acc(Soltype::ITR,1,activity.as_mut_slice())?;
    println!("Activity of quadratic ACC:: {:?}",activity);

    //TAG:begin-getdoty
    // Demonstrate retrieving the dual of ACC
    task.get_acc_dot_y(Soltype::ITR,1,doty.as_mut_slice())?;
    println!("Dual of quadratic ACC:: {:?}",doty);
    //TAG:end-getdoty

    //TAG:ASSERT:begin-check-solution
    //maxgap = lambda a, b: max(abs(x-y) for x,y in zip(a,b))
    let compl : f64 = dot(activity.as_slice(),doty.as_slice());
    assert! (compl.abs() < 1e-7);
    assert! (maxgap(xx.as_slice(),       &[-0.07838011145615721, 1.1289128998004547, -0.0505327883442975]) < 1e-7);
    assert! (maxgap(doty.as_slice(),     &[-1.9429680870375095, -0.30303030303030304, -1.9191919191919191]) < 1e-7);
    assert! (maxgap(activity.as_slice(), &[0.03, -0.004678877204190343, -0.029632888959872067]) < 1e-7);
    println!("Complementarity {}",compl);
    //TAG:ASSERT:end-check-solution

    Ok(())
}

fn maxgap(a : &[f64], b : &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&a,&b)| (a - b).abs()).max_by(|a,b| if a < b { std::cmp::Ordering::Less } else if b < a { std::cmp::Ordering::Greater } else {std::cmp::Ordering::Equal} ).unwrap()
}
fn dot(a : &[f64], b : &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(&a,&b)| (a * b)).sum()
}
