/*
   Copyright: Copyright (c) MOSEK ApS, Denmark. All rights reserved.

   File:      sparsecholesky.rs

   Purpose:   Demonstrate the sparse Cholesky factorization.

 */
extern crate mosek;
extern crate itertools;
use mosek::Transpose;
use itertools::{iproduct,izip};

fn print_sparse(n     : usize,
                perm  : &[i32],
                diag  : &[f64],
                lnzc  : &[i32],
                lptrc : &[i64],
                lsubc : &[i32],
                lvalc : &[f64]) {
    println!("P = {:?}",perm);
    println!("diag(D) = {:?}",diag);

    let mut l = vec![0.0; n*n];

    for (j,(&ptr,&sz)) in lptrc.iter().zip(lnzc.iter()).enumerate() {
        let pfrom = ptr as usize;
        let pto = ptr as usize + sz as usize;
        for (&subci,&valci) in lsubc[pfrom..pto].iter().zip(lvalc[pfrom..pto].iter()) {
            l[subci as usize * n + j] = valci;
        }
    };
    println!("L = [");
    (0..n*n).step_by(n).for_each(|i| {
        print!("  {:?}",&l[i..i+n]);
    });
    println!("  ]");
}


fn main() -> Result<(),String> {
    /* Create the mosek environment. */
    //Example from the manual
    //Observe that anzc, aptrc, asubc and avalc only specify the lower triangular part.
    let n     = 4;
    let anzc  = [4, 1, 1, 1];
    let asubc = [0, 1, 2, 3, 1, 2, 3];
    let aptrc = [0, 4, 5, 6];
    let avalc = [4.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let b     = [13.0, 3.0, 4.0, 5.0];

    let mut perm   = Vec::new();
    let mut lnzc   = Vec::new();
    let mut lsubc  = Vec::new();
    let mut diag   = Vec::new();
    let mut lptrc  = Vec::new();
    let mut lvalc  = Vec::new();
    let mut lensubnval = 0;

    mosek::compute_sparse_cholesky(0,        //Mosek chooses number of threads
                                   1,        //Apply reordering heuristic
                                   1.0e-14,  //Singularity tolerance
                                   &anzc, &aptrc, &asubc, &avalc,
                                   & mut perm, & mut diag,
                                   & mut lnzc, & mut lptrc, & mut lensubnval, & mut lsubc, & mut lvalc)?;
    print_sparse(n, perm.as_slice(), diag.as_slice(), lnzc.as_slice(), lptrc.as_slice(), lsubc.as_slice(), lvalc.as_slice());

    /* Permuted b is stored as x. */
    let mut x : Vec<f64> = perm.iter().map(|&i| b[i as usize]).collect();

    /*Compute  inv(L)*x.*/
    mosek::sparse_triangular_solve_dense(Transpose::NO,  lnzc.as_slice(), lptrc.as_slice(), lsubc.as_slice(), lvalc.as_slice(), x.as_mut_slice())?;
    /*Compute  inv(L^T)*x.*/
    mosek::sparse_triangular_solve_dense(Transpose::YES, lnzc.as_slice(), lptrc.as_slice(), lsubc.as_slice(), lvalc.as_slice(), x.as_mut_slice())?;

    print!("\nSolution A x = b, x = [ {:?} ]",
           iproduct!(0..n,izip!(perm.iter(),x.iter()))
           .filter_map(|(i,(&pj,&xj))|if pj as usize == i { Some(xj) } else { None })
           .collect::<Vec<f64>>());
    let n     = 3;
    let anzc  = [3, 2, 1];
    let asubc = [0, 1, 2, 1, 2, 2];
    let aptrc = [0, 3, 5, ];
    let avalc = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let mut perm   = Vec::new();
    let mut lnzc   = Vec::new();
    let mut lsubc  = Vec::new();
    let mut diag   = Vec::new();
    let mut lptrc  = Vec::new();
    let mut lvalc  = Vec::new();
    let mut lensubnval = 0;

    mosek::compute_sparse_cholesky(0,        //Mosek chooses number of threads
                                   1,        //Apply reordering heuristic
                                   1.0e-14,  //Singularity tolerance
                                   &anzc, &aptrc, &asubc, &avalc,
                                   & mut perm, & mut diag,
                                   & mut lnzc, & mut lptrc, & mut lensubnval, & mut lsubc, & mut lvalc)?;

    print_sparse(n, perm.as_slice(), diag.as_slice(), lnzc.as_slice(), lptrc.as_slice(), lsubc.as_slice(), lvalc.as_slice());

    Ok(())
}
