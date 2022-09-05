//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : solvelinear.rs
//!
//!   Purpose :   To demonstrate the usage of MSK_solvewithbasis
//!               when solving the linear system:
//!
//!               1.0  x1             = b1
//!               -1.0  x0  +  1.0  x1 = b2
//!
//!               with two different right hand sides
//!
//!               b = (1.0, -2.0)
//!
//!               and
//!
//!               b = (7.0, 0.0)


extern crate mosek;

use mosek::{Task,Boundkey,Streamtype,Soltype,Stakey};

const INF : f64 = 0.0;

fn setup(task : & mut mosek::Task,
         aval : &[f64],
         asub : &[i32],
         ptrb : &[i64],
         ptre : &[i64],
         numvar : i32) -> Result<Vec<i32>,String> {

    // mosek.stakey[] skx = new mosek.stakey [numvar];
    // mosek.stakey[] skc = new mosek.stakey [numvar];

    // for (int i = 0; i < numvar ; ++i) {
    //   skx[i] = mosek.stakey.bas;
    //   skc[i] = mosek.stakey.fix;
    // }

    task.append_vars(numvar)?;
    task.append_cons(numvar)?;

    task.put_a_col_slice(0,numvar,ptrb,ptre,asub,aval)?;

    task.put_con_bound_slice_const(0,numvar,Boundkey::FX,0.0,0.0)?;
    task.put_var_bound_slice_const(0,numvar,Boundkey::FR,-INF,INF)?;

    /* Define a basic solution by specifying
       status keys for variables & constraints. */
    task.delete_solution(Soltype::BAS)?;

    task.put_skc_slice(Soltype::BAS, 0, numvar, vec![Stakey::FIX; numvar as usize].as_slice())?;
    task.put_skx_slice(Soltype::BAS, 0, numvar, vec![Stakey::BAS; numvar as usize].as_slice())?;

    let mut basis = vec![0; numvar as usize];
    task.init_basis_solve(basis.as_mut_slice())?;
    Ok(basis)
  }

fn main() -> Result<(),String> {
    let numcon : i32 = 2;
    let numvar : i32 = 2;

    let aval = &[ -1.0 ,
                   1.0, 1.0 ];
    let asub = &[ 1,
                  0,   1   ];
    let ptrb = &[0, 1];
    let ptre = &[1, 3];

    // int[]       bsub  = new int[numvar];
    // double[]    b     = new double[numvar];
    // int[]       basis = new int[numvar];

    let mut task = Task::new().unwrap();

    // Put A matrix and factor A.  Call this function only once for a
    // given task.

    let basis = setup(& mut task,
                      aval,
                      asub,
                      ptrb,
                      ptre,
                      numvar)?;

    let mut task = task.with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
    
    /* now solve rhs */
    let mut b    = vec![0.0; numvar as usize];
    let mut bsub = vec![0;   numvar as usize];

    b[0]    = 1.0; b[1]    = -2.0;
    bsub[0] = 0;   bsub[1] = 1;
    let nz = task.solve_with_basis(false, 2, bsub.as_mut_slice(), b.as_mut_slice())?;
    println!("\nSolution to Bx = b:\n");

    // Print solution and show correspondents to original variables in
    // the problem
    for &bsubi in bsub[..nz as usize].iter() {
        if basis[bsubi as usize] < numcon {
            println!("This should never happen");
        }
        else {
            println!("x{} = {}",basis[bsubi as usize] - numcon, b[bsubi as usize]);
        }
    }

    b[0]    = 7.0;
    bsub[0] = 0;
    let nz = task.solve_with_basis(false, 1, bsub.as_mut_slice(), b.as_mut_slice())?;

    println!("Solution to Bx = b:");
    // Print solution and show correspondents to original variables in
    // the problem
    for &bsubi in bsub[..nz as usize].iter() {
        if basis[bsubi as usize] < numcon {
            println!("This should never happen");
        }
        else {
            println!("x{} = {}",basis[bsubi as usize] - numcon,b[bsubi as usize]);
        }
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
