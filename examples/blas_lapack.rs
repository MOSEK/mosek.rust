//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : blas_lapack.rs
//!
//!  Purpose: To demonstrate how to call BLAS/LAPACK routines for
//!           which MOSEK provides simplified interfaces.
//!
extern crate mosek;

fn print_matrix(x : &[f64], r : i32, c : i32) {
    let mut k = 0usize;
    print!("[");
    println!(" {:?}",&x[k..k+c as usize]); k += c as usize;
    for _i in 1..r  {
        println!("  {:?}",&x[k..k+c as usize]);
    }
    println!("]");
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {

    let n = 3i32;
    let m = 2i32;
    let k = 3i32;

    let alpha = 2.0;
    let beta  = 0.5;
    let x    = &[1.0, 1.0, 1.0];
    let mut y = vec![1.0, 2.0, 3.0];
    let mut z = vec![1.0, 1.0];
    let A    = &[1.0, 1.0, 2.0, 2.0, 3.0, 3.0];
    let B    = &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
    let mut C = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mut D = vec![1.0, 1.0, 1.0, 1.0];
    let mut Q = vec![1.0, 0.0, 0.0, 2.0];
    let mut v = vec![0.0, 0.0, 0.0];

    let mut xy : f64 = 0.0;

    /* BLAS routines*/
    println!("n={} m={} k={}", m, n, k);
    println!("alpha={}", alpha);
    println!("beta={}", beta);

    mosek::dot(n,x,y.as_slice(),& mut xy)?;
    println!("dot results = {}\n", xy);

    print_matrix(x, 1, n);
    print_matrix(y.as_slice(), 1, n);

    mosek::axpy(n, alpha, x, y.as_mut_slice())?;
    println!("axpy results is:\n");
    print_matrix(y.as_slice(), 1, n);


    mosek::gemv(mosek::Transpose::NO, m, n, alpha, A, x, beta, z.as_mut_slice())?;
    println!("gemv results is:");
    print_matrix(z.as_slice(), 1, m);

    mosek::gemm(mosek::Transpose::NO, mosek::Transpose::NO, m, n, k, alpha, A, B, beta, C.as_mut_slice())?;
    println!("gemm results is");
    print_matrix(C.as_slice(), m, n);

    mosek::syrk(mosek::Uplo::LO, mosek::Transpose::NO, m, k, 1., A, beta, D.as_mut_slice())?;
    println!("syrk results is");
    print_matrix(D.as_slice(), m, m);

    /* LAPACK routines*/

    mosek::potrf(mosek::Uplo::LO, m, Q.as_mut_slice())?;
    println!("potrf results is");
    print_matrix(Q.as_slice(), m, m);

    mosek::syeig(mosek::Uplo::LO, m, Q.as_slice(), & mut v[0..m as usize])?;
    println!("syeig results is");
    print_matrix(v.as_slice(), 1, m);

    mosek::syevd(mosek::Uplo::LO, m, Q.as_mut_slice(), &mut v[0..m as usize])?;
    println!("syevd results is");
    print_matrix(v.as_slice(), 1, m);
    print_matrix(Q.as_slice(), m, m);

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
