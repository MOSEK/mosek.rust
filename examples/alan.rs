extern crate mosek;
use mosek::model::*;
//
// Copyright: $$copyright
//
//  File:     $${file}
//
//  Purpose: This file contains an implementation of the alan.gms (as
//  found in the GAMS online model collection) using Java/MBI.
//
//  The model is a simple portfolio choice model. The objective is to
//  invest in a number of assets such that we minimize the risk, while
//  requiring a certain expected return.
//
//  We operate with 4 assets (hardware,software, show-biz and treasure
//  bill). The risk is defined by the covariance matrix
//    Q = [[  4.0, 3.0, -1.0, 0.0 ],
//         [  3.0, 6.0,  1.0, 0.0 ],
//         [ -1.0, 1.0, 10.0, 0.0 ],
//         [  0.0, 0.0,  0.0, 0.0 ]]
//
//
//  We use the form Q = U^T * U, where U is a Cholesky factor of Q.
//


fn main() -> Result<(),String> {
    /////////////////////////////////////////////////////////////////////
    // Problem data.
    // Security names

    let securities = vec!["hardware", "software", "show-biz", "t-bills"];
    // Mean returns on securities
    let mean       = vec![       8.0,        9.0,       12.0,       7.0];
    // Target mean return
    let target = 10.0;
    let numsec = securities.len();

    // Factor of covariance matrix.
    let U : (usize,usize,Vec<f64>) =
        (numsec,numsec,vec![ 2.0       ,  1.5       , -0.5       , 0.0,
                             0.0       ,  1.93649167,  0.90369611, 0.0,
                             0.0       ,  0.0       ,  2.98886824, 0.0,
                             0.0       ,  0.0       ,  0.0       , 0.0 ]);
    let mut M = Model::with_name("alan")?;

    let x = M.variable("x", &greater_than(vec![0.0; numsec]))?;
    let t = M.variable("t", &greater_than_scalar(0.0))?;
    M.objective("minvar", ObjectiveSense::Min, &t)?;

    // sum securities to 1.0
    M.constraint("wealth",
                 &expr_sum(&x),
                 &equal_to_scalar(1.0))?;
    // define target expected return
    M.constraint("dmean",
                 &expr_dot(mean.as_slice(), &x),
                 &greater_than_scalar(target))?;
    M.constraint("t > ||Ux||_2",
                 &expr_stack_3(&0.5, &t, &expr_mul(&U, &x)),
                 &in_rotated_second_order_cone(4))?;

    //M.setLogHandler(new java.io.PrintWriter(System.out));

    println!("Solve...");
    M.solve()?;

    M.write_task("alan.ptf")?;
    println!("... Solved.");

    let mut solx = vec![0.0; numsec];
    M.level(&x,solx.as_mut_slice())?;

    println!("Solution = {:?}", solx);

    Ok(())
}
