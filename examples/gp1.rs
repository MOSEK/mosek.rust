//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : gp1.rs
//!
//!   Purpose:   Demonstrates how to solve a simple Geometric Program (GP)
//!              cast into conic form with exponential cones and log-sum-exp.
//!
//!              Example from
//!                https://gpkit.readthedocs.io/en/latest/examples.html//maximizing-the-volume-of-a-box
//!
extern crate mosek;

use mosek::{Task,Boundkey,Objsense,Streamtype,Soltype};

// Since the value of infinity is ignored, we define it solely
// for symbolic purposes
const INF : f64 = 0.0;

#[allow(non_snake_case)]
fn max_volume_box(Aw : f64,
                  Af : f64,
                  alpha : f64,
                  beta : f64,
                  gamma : f64,
                  delta : f64) -> Result<Vec<f64>,String>
{
    let numvar = 3i32;  // Variables in original problem
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        }.with_callbacks();

    // Directs the log task stream to the user specified
    // method task_msg_obj.stream
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // Add variables and constraints
    task.append_vars(numvar)?;

    let x = 0i32;
    let y = 1i32;
    let z = 2i32;

    // Objective is the sum of three first variables
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    task.put_c_slice(0, numvar, &[1.0,1.0,1.0])?;

    task.put_var_bound_slice_const(0, numvar, Boundkey::FR, -INF, INF)?;

    task.append_cons(3)?;
    // s0+s1 < 1 <=> log(s0+s1) < 0
    task.put_aij_list(&[0,0,1,1,2,2],
                      &[y, z, x, y, z, y],
                      &[1.0, 1.0, 1.0, -1.0, 1.0, -1.0])?;

    task.put_con_bound(0,Boundkey::UP,-INF,Af.ln())?;
    task.put_con_bound(1,Boundkey::RA,alpha.ln(),beta.ln())?;
    task.put_con_bound(2,Boundkey::RA,gamma.ln(),delta.ln())?;


    {
        let afei = task.get_num_afe()?;
        let u1 = task.get_num_var()?;
        let u2 = u1+1;

        let afeidx = &[0, 1, 2, 2, 3, 3, 5, 5];
        let varidx = &[u1, u2, x, y, x, z, u1, u2];
        let fval   = &[1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];
        let gfull  = &[0.0, 0.0, (2.0/Aw).ln(), (2.0/Aw).ln(), 1.0, -1.0];

        task.append_vars(2)?;
        task.append_afes(6)?;

        task.put_var_bound_slice_const(u1, u1+2, Boundkey::FR, -INF, INF)?;

        // Affine expressions appearing in affine conic constraints
        // in this order:
        // u1, u2, x+y+log(2/Awall), x+z+log(2/Awall), 1.0, u1+u2-1.0
        task.put_afe_f_entry_list(afeidx, varidx, fval)?;
        task.put_afe_g_slice(afei, afei+6, gfull)?;
        {
            let dom = task.append_primal_exp_cone_domain()?;

            // (u1, 1, x+y+log(2/Awall)) \in EXP
            task.append_acc(dom, &[0, 4, 2], &[0.0,0.0,0.0])?;

            // (u2, 1, x+z+log(2/Awall)) \in EXP
            task.append_acc(dom, &[1, 4, 3], &[0.0,0.0,0.0])?;
        }
        {
            let dom = task.append_rzero_domain(1)?;
            // The constraint u1+u2-1 \in \ZERO is added also as an ACC
            task.append_acc(dom, &[5], &[0.0])?;
        }
    }

    let _trm = task.optimize()?;
    task.write_data("gp1.ptf")?;
    let mut xyz = vec![0.0; 3];
    task.get_xx_slice(Soltype::ITR, 0i32, numvar, xyz.as_mut_slice())?;

    // task.write_data("gp1.ptf")?;

    Ok(xyz.iter().map(|v| v.exp()).collect())
}

#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    // maximize     h*w*d
    // subjecto to  2*(h*w + h*d) <= Awall
    //              w*d <= Afloor
    //              alpha <= h/w <= beta
    //              gamma <= d/w <= delta
    //
    // Variable substitutions:  h = exp(x), w = exp(y), d = exp(z).
    //
    // maximize     x+y+z
    // subject      log( exp(x+y+log(2/Awall)) + exp(x+z+log(2/Awall)) ) <= 0
    //                              y+z <= log(Afloor)
    //              log( alpha ) <= x-y <= log( beta )
    //              log( gamma ) <= z-y <= log( delta )
    //
    // Finally, the model we will implement:
    //
    // maximize   x+y+z
    // subject to s0 > exp(x+y+log(2/Awall); (s0,1,x+y+log(2/Awall)) in PEXP
    //            s1 > exp(x+z+log(2/Awall); (s1,1,x+z+log(2/Awall)) in PEXP
    //            s0+s1 < 1
    //
    //            y+z < log Afloor
    //
    //            x-y in [log alpha; log beta]
    //            z-y in [log gamma; log delta]
    //

    // (x,y,z) in pexp : x0 > x1 * exp(x2/x1)

    let Aw    = 200.0;
    let Af    = 50.0;
    let alpha = 2.0;
    let beta  = 10.0;
    let gamma = 2.0;
    let delta = 10.0;

    let hwd = max_volume_box(Aw, Af, alpha, beta, gamma, delta)?;

    println!("h={:.4} w={:.4} d={:.4}\n", hwd[0], hwd[1], hwd[2]);

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
