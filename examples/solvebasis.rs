//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : solvebasis.rs
//!

extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype};

/// Demonstrate the usage of solve_with_basis on the problem:
///
/// ```
/// maximize  x0 + x1
/// st.
///         x0 + 2.0 x1 <= 2
///         x0  +    x1 <= 6
///         x0 >= 0, x1>= 0
/// ```
///
/// The problem has the slack variables `xc0`, `xc1` on the
/// constraints and the variabels `x0` and `x1`.
///
/// ```
/// maximize  x0 + x1
/// st.
///     x0 + 2.0 x1 -xc1       = 2
///     x0  +    x1       -xc2 = 6
///     x0 >= 0, x1>= 0,
///     xc1 <=  0 , xc2 <= 0
/// ```
///
fn solve() -> Result<(),String> {
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    }.with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    task.put_obj_name("solvebasis")?;

    let numcon : i32 = 2;
    let numvar : i32 = 2;

    let mut w1  = vec![2.0, 6.0];
    let mut w2  = vec![1.0, 0.0];

    task.input_data(numcon, numvar,
                    &[1.0, 1.0], // c
                    0.0, // cfix
                    &[0,2], // ptrb
                    &[2,3], // ptre
                    &[0,1,
                      0,1],  // sub
                    &[1.0, 1.0,
                      2.0, 1.0], // val
                    &[Boundkey::UP,
                      Boundkey::UP], // bkc
                    &[0.0,0.0], // blc
                    &[2.0,6.0], // buc
                    &[Boundkey::LO,
                      Boundkey::LO], // bkx
                    &[0.0, 0.0], // blx
                    &[0.0,0.0])?; // bux;

    task.put_obj_sense(Objsense::MAXIMIZE)?;

    let _ = task.optimize()?;

    let mut basis = vec![0i32; numcon as usize];
    task.init_basis_solve(basis.as_mut_slice())?;

    // List basis variables corresponding to columns of B
    let mut varsub = vec![0i32, 1];

    for i in 0..numcon {
        if basis[varsub[i as usize] as usize] < numcon {
            println!("Basis variable no {} is xc{}",i,basis[varsub[i as usize] as usize]);
        }
        else {
            println!("Basis variable no {} is x{}",i,basis[i as usize]-numcon);

            // solve Bx = w1
            // varsub contains index of non-zeros in b.
            //  On return b contains the solution x and
            // varsub the index of the non-zeros in x.
            {
                let nz = task.solve_with_basis(false, 2, varsub.as_mut_slice(), w1.as_mut_slice())?;
                println!("nz = {}",nz);
                println!("Solution to Bx = {:?}",w1);

                for vsubi in &varsub[0..nz as usize] {
                    if basis[*vsubi as usize] < numcon {
                        println!("xc {} = {}",basis[*vsubi as usize],w1[*vsubi as usize]);
                    }
                    else {
                        println!("x{} = {}",basis[*vsubi as usize] - numcon,w1[*vsubi as usize])
                    }
                }
            }

            // Solve B^Tx = w2
            {
                varsub[0] = 1;
                let nz = task.solve_with_basis(true,1,varsub.as_mut_slice(),w2.as_mut_slice())?;
                println!("nz = {}",nz);

                println!("Solution to B^Tx = {:?}",w2);

                for vsubi in &varsub[0..nz as usize] {
                    if basis[*vsubi as usize] < numcon {
                        print!("xc{} = {}",basis[*vsubi as usize],w2[*vsubi as usize]);
                    }
                    else {
                        print!("x{} = {}",basis[*vsubi as usize] - numcon,w2[*vsubi as usize]);
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(),String> {
    solve()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
