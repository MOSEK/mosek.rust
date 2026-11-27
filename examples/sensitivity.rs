//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : sensitivity.rs
//!
//!   Purpose :   To demonstrate how to perform sensitivity
//!               analysis from the API on a small problem:
//!
//!               minimize
//!
//!               obj: +1 x11 + 2 x12 + 5 x23 + 2 x24 + 1 x31 + 2 x33 + 1 x34
//!               st
//!               c1:   +  x11 +   x12                                           <= 400
//!               c2:                  +   x23 +   x24                           <= 1200
//!               c3:                                  +   x31 +   x33 +   x34   <= 1000
//!               c4:   +  x11                         +   x31                   = 800
//!               c5:          +   x12                                           = 100
//!               c6:                  +   x23                 +   x33           = 500
//!               c7:                          +   x24                 +   x34   = 500
//!
//!               The example uses basis type sensitivity analysis.
//!

extern crate mosek;
use mosek::{Task,Boundkey,Streamtype,Mark,Objsense};

const INFINITY : f64 = 0.0;

fn main() -> Result<(),String> {
    let bkc = vec![
        Boundkey::UP, Boundkey::UP,
        Boundkey::UP, Boundkey::FX,
        Boundkey::FX, Boundkey::FX,
        Boundkey::FX ];
    let bkx = vec![
        Boundkey::LO, Boundkey::LO,
        Boundkey::LO, Boundkey::LO,
        Boundkey::LO, Boundkey::LO,
        Boundkey::LO ];

    let ptr = [0i64, 2, 4, 6, 8, 10, 12, 14];
    let sub = [0i32, 3, 0, 4, 1, 5, 1, 6, 2, 3, 2, 5, 2, 6];
    let blc = [ -INFINITY, -INFINITY, -INFINITY, 800.0, 100.0, 500.0, 500.0 ];

    let buc = [400.0, 1200.0, 1000.0, 800.0, 100.0, 500.0, 500.0];
    let c   = [1.0, 2.0, 5.0, 2.0, 1.0, 2.0, 1.0];
    let blx = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let bux = [INFINITY, INFINITY,
               INFINITY, INFINITY,
               INFINITY, INFINITY,
               INFINITY];
    let val = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
               1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];

    let numcon = 7;  /* Number of constraints.             */
    let numvar = 7;  /* Number of variables.               */

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(t) => t,
        None => return Err("Failed to create task".to_string()),
    }.with_callbacks();

    /* Directs the log task stream to the 'printstr' function. */
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    task.input_data(numcon as i32, numvar as i32,
                    &c,
                    0.0,
                    &ptr[0..numvar as usize],
                    &ptr[1..numvar as usize+1],
                    &sub,
                    &val,
                    &bkc,
                    &blc,
                    &buc,
                    &bkx,
                    &blx,
                    &bux)?;

    /* A maximization problem */
    task.put_obj_sense(Objsense::MINIMIZE)?;

    task.optimize()?;

    /* Analyze upper bound on c1 and the equality constraint on c4 */
    let mut subi  = vec![0i32, 3i32];
    let mut marki = vec![Mark::UP, Mark::UP];

    /* Analyze lower bound on the variables x12 and x31 */
    let mut subj  = vec![1i32, 4];
    let mut markj = vec![Mark::LO, Mark::LO];

    let mut leftpricei  = vec![0.0; 2];
    let mut rightpricei = vec![0.0; 2];
    let mut leftrangei  = vec![0.0; 2];
    let mut rightrangei = vec![0.0; 2];
    let mut leftpricej  = vec![0.0; 2];
    let mut rightpricej = vec![0.0; 2];
    let mut leftrangej  = vec![0.0; 2];
    let mut rightrangej = vec![0.0; 2];

    task.primal_sensitivity(subi.as_mut_slice(),
                            marki.as_mut_slice(),
                            subj.as_mut_slice(),
                            markj.as_mut_slice(),
                            leftpricei.as_mut_slice(),
                            rightpricei.as_mut_slice(),
                            leftrangei.as_mut_slice(),
                            rightrangei.as_mut_slice(),
                            leftpricej.as_mut_slice(),
                            rightpricej.as_mut_slice(),
                            leftrangej.as_mut_slice(),
                            rightrangej.as_mut_slice())?;
    println!("Results from sensitivity analysis on bounds:");

    println!("For constraints:");
    for i in 0..2 {
        println!("leftprice = {:.5e}, rightprice = {:.5e}, leftrange = {:.5e}, rightrange = {:.5e}",
                 leftpricei[i], rightpricei[i], leftrangei[i], rightrangei[i]);
    }
    println!("For variables:\n");
    for i in 0..2 {
        println!("leftprice = {:.5e}, rightprice = {:.5e}, leftrange = {:.5e}, rightrange = {:.5e}",
                 leftpricej[i], rightpricej[i], leftrangej[i], rightrangej[i]);
    }

    let mut leftprice  = vec![0.0; 2];
    let mut rightprice = vec![0.0; 2];
    let mut leftrange  = vec![0.0; 2];
    let mut rightrange = vec![0.0; 2];
    let subc = [2i32, 5i32];

    task.dual_sensitivity(&subc,
                          leftprice.as_mut_slice(),
                          rightprice.as_mut_slice(),
                          leftrange.as_mut_slice(),
                          rightrange.as_mut_slice())?;

    println!("Results from sensitivity analysis on objective coefficients:");

    for i in 0..2 {
        println!("leftprice = {:.5e}, rightprice = {:.5e}, leftrange = {:.5e}, rightrange = {:.5e}",
                 leftprice[i], rightprice[i], leftrange[i], rightrange[i]);
    }

    return Result::Ok(());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
