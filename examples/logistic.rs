//!
//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! File : logistic.rs
//!
//! Purpose: Implements logistic regression with regulatization.
//!
//!          Demonstrates using the exponential cone and log-sum-exp in Optimizer API.
//!
//!          Plots an example for 2D datasets.

extern crate mosek;
extern crate itertools;

use mosek::{Task,TaskCB,Boundkey,Objsense,Soltype,Streamtype};
use itertools::{izip,iproduct};

const INF : f64 = 0.0;

#[allow(non_snake_case)]
fn softplus(task : & mut TaskCB, d : i32, n : i32, theta : i32, t : i32, X : &[f64], Y : &[bool]) -> Result<(),String> {
    let nvar = task.get_num_var()?;
    let ncon = task.get_num_con()?;
    let nafe = task.get_num_afe()?;
    task.append_vars(2*n)?;   // z1, z2
    task.append_cons(n)?;     // z1 + z2 = 1
    task.append_afes(4*n as i64)?;   //theta * X[i] - t[i], -t[i], z1[i], z2[i]
    let z1 = nvar;
    let z2 = nvar+n;
    let zcon = ncon;
    let thetaafe = nafe;
    let tafe = nafe+n   as i64;
    let z1afe = tafe+n  as i64;
    let z2afe = z1afe+n as i64;

    // Linear constraints
    {
        let mut subi = vec![0i32; 2*n as usize];
        let mut subj = vec![0i32; 2*n as usize];
        let mut aval = vec![0.0;  2*n as usize];

        for i in 0..n {
            task.put_var_name(z1+i,format!("z1[{}]",i).as_str())?;
            task.put_var_name(z2+i,format!("z2[{}]",i).as_str())?;
        }
        for ((i,&zx),si, sj, av) in izip!(iproduct!(0..n,&[z1,z2]),
                                          subi.iter_mut(),
                                          subj.iter_mut(),
                                          aval.iter_mut()) {
            *si = zcon+i;
            *sj = zx+i as i32;
            *av = 1.0;
        }

        task.put_aij_list(subi.as_slice(), subj.as_slice(), aval.as_slice())?;
        task.put_con_bound_slice_const(zcon, zcon+n, Boundkey::FX, 1.0, 1.0)?;
        task.put_var_bound_slice_const(nvar, nvar+2*n, Boundkey::FR, -INF, INF)?;
    }
    // Affine conic expressions
    let mut afeidx = vec![0i64; (d*n+4*n) as usize];
    let mut varidx = vec![0i32; (d*n+4*n) as usize];
    let mut fval   = vec![0.0;  (d*n+4*n) as usize];

    // Thetas
    let mut k : usize = 0;
    for ((i,j),afei,vari) in izip!(iproduct!(0..n,0..d),
                                   & mut afeidx[k..k+(n*d) as usize],
                                   & mut varidx[k..k+(n*d) as usize]) {
        *afei = thetaafe + i as i64;
        *vari = theta    + j;
    }

    for ((&yi,_j),&xij,fv) in izip!(iproduct!(Y,0..d), X, & mut fval[k..k+(n*d) as usize]) {
        *fv = (if yi {-1.0} else {1.0}) * xij;
    }
    k += (n*d) as usize;

    for fv in fval[k..k+(2*n) as usize].iter_mut() { *fv = -1.0; }
    for (i,afei,vari) in izip!(0..n,
                               &mut afeidx[k..k+(2*n) as usize].iter_mut().step_by(2),
                               &mut varidx[k..k+(2*n) as usize].iter_mut().step_by(2)) {
        *afei = thetaafe+i as i64; *vari = t+i;
    }
    for (i,afei,vari) in izip!(0..n,
                               &mut afeidx[k+1..k+(2*n) as usize].iter_mut().step_by(2),
                               &mut varidx[k+1..k+(2*n) as usize].iter_mut().step_by(2)) {
        *afei = tafe+i as i64; *vari = t+i;
    }
    k += (n*2) as usize;

    for fv in fval[k..k+(2*n) as usize].iter_mut() { *fv = 1.0; }
    for (i,afei,vari) in izip!(0..n,
                               &mut afeidx[k..k+(2*n) as usize].iter_mut().step_by(2),
                               &mut varidx[k..k+(2*n) as usize].iter_mut().step_by(2)) {
        *afei = z1afe+i as i64; *vari = z1+i;
    }
    for (i,afei,vari) in izip!(0..n,
                               &mut afeidx[k+1..k+(2*n) as usize].iter_mut().step_by(2),
                               &mut varidx[k+1..k+(2*n) as usize].iter_mut().step_by(2)) {
        *afei = z2afe+i as i64; *vari = z2+i;
    }


    // Add the expressions
    task.put_afe_f_entry_list(afeidx.as_slice(), varidx.as_slice(), fval.as_slice())?;

    {
        // Add a single row with the constant expression "1.0"
        let oneafe = task.get_num_afe()?;
        task.append_afes(1)?;
        task.put_afe_g(oneafe, 1.0)?;

        // Add an exponential cone domain
        let dom = task.append_primal_exp_cone_domain()?;

        // Conic constraints
        let zeros = &[0.0,0.0,0.0];
        let mut acci = task.get_num_acc()?;
        for i in 0..n as i64 {
            task.append_acc(dom, &[z1afe+i, oneafe, thetaafe+i], zeros)?;
            task.append_acc(dom, &[z2afe+i, oneafe, tafe+i],     zeros)?;
            task.put_acc_name(acci,format!("z1:theta[{}]",i).as_str())?;
            task.put_acc_name(acci+1,format!("z2:t[{}]",i).as_str())?;

            acci += 2;
        }
    }

    Ok(())
}

  // Model logistic regression (regularized with full 2-norm of theta)
  // X - n x d matrix of data points
  // y - length n vector classifying training points
  // lamb - regularization parameter
#[allow(non_snake_case)]
fn logistic_regression(X : &[f64],
                       Y : &[bool],
                       lamb : f64) -> Result<Vec<f64>,String> {
    let n = Y.len() as i32;
    let d = (X.len()/Y.len()) as i32;  // num samples, dimension

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // Variables [r; theta; t]
    let nvar : i32 = 1+d+n;
    task.append_vars(nvar)?;
    task.put_var_bound_slice_const(0, nvar, Boundkey::FR, -INF, INF)?;
    let (r,theta,t) = (0i32,1i32,1+d);
    task.put_var_name(r,"r")?;
    for j in 0..d { task.put_var_name(theta+j,format!("theta[{}]",j).as_str())?; }
    for j in 0..n { task.put_var_name(t+j,format!("t[{}]",j).as_str())?; }

    // Objective lambda*r + sum(t)
    task.put_c_j(r, lamb)?;
    for i in 0..n { task.put_c_j(t+i, 1.0)?; }
    task.put_obj_sense(Objsense::MINIMIZE)?;

    // Softplus function constraints
    softplus(& mut task, d, n, theta, t, X, Y)?;

    // Regularization
    // Append a sequence of linear expressions (r, theta) to F
    let numafe = task.get_num_afe()?;
    task.append_afes(1+d as i64)?;
    task.put_afe_f_entry(numafe, r, 1.0)?;
    for i in 0..d {
        task.put_afe_f_entry(numafe + i as i64 + 1, theta + i, 1.0)?;
    }

    // Add the constraint
    {
        let dom = task.append_quadratic_cone_domain((1+d) as i64)?;
        task.append_acc_seq(dom,
                            numafe,
                            vec![0.0; 1+d as usize].as_slice())?;
    }
    // Solution
    task.write_data("logistic.ptf")?;
    task.optimize()?;

    let mut xx = vec![0.0; d as usize];
    task.get_xx_slice(Soltype::ITR, theta, theta+d as i32,xx.as_mut_slice())?;
    Ok(xx)
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    // Test: detect and approximate a circle using degree 2 polynomials
    let n : i32 = 30;
    let d : i32 = 2;
    let (X,Y) = {
        let mut Y = vec![false;(n*n) as usize];
        let mut x1 = vec![0.0;(n*n) as usize];
        let mut x2 = vec![0.0;(n*n) as usize];
        let step = 2.0/((n-1) as f64);
        for (x1i,x2i,yi,(z1,z2)) in izip!(x1.iter_mut(),x2.iter_mut(),Y.iter_mut(),
                                          iproduct!((0..30).map(|i| -1.0 + (i as f64) * step),
                                                    (0..30).map(|i| -1.0 + (i as f64) * step))) {
            *x1i = z1; *x2i = z2; *yi = z1*z1+z2*z2 >= 0.69;
        }

        let X : Vec<f64> = iproduct!(izip!(x1,x2),iproduct!(0..d+1,0..d+1)).filter_map(|((p0,p1),(a,b))| if a+b <= d {Some(f64::powi(p0,a)*f64::powi(p1,b))} else {None}).collect();

        (X,Y)
    };

    let theta = logistic_regression( X.as_slice(), Y.as_slice(), 0.1)?;

    println!("theta = {:?}",theta);

    return Result::Ok(());
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
