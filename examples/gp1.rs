//
//   Copyright: Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//
//   File:      $${file}
//
//   Purpose:   Demonstrates how to solve a simple Geometric Program (GP)
//              cast into conic form with exponential cones and log-sum-exp.
//
//              Example from
//                https://gpkit.readthedocs.io/en/latest/examples.html//maximizing-the-volume-of-a-box
//
extern crate mosek;

// Since the value of infinity is ignored, we define it solely
// for symbolic purposes
const INF : f64 = 0.0;

fn max_volume_box(Aw : f64, Af : f64,
                  alpha : f64, beta : f64, gamma : f64, delta : f64) -> Result<Vec<f64>,String>
{
    let numvar = 3i32;  // Variables in original problem
    /* Create the optimization task. */
    let mut task = match mosek::Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        };

    // Directs the log task stream to the user specified
    // method task_msg_obj.stream
    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;

    // Add variables and constraints
    task.append_vars(numvar+2)?;
    task.append_cons(4)?;

    let xx = &[0i32,1i32,2i32];
    let s  = &[3i32,4i32];

    // Objective is the sum of three first variables
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE)?;
    task.put_c_slice(0, numvar, &[1.0,1.0,1.0])?;

    task.put_var_bound_slice_const(0, numvar+2, mosek::MSK_BK_FR, -INF, INF)?;

    task.append_afes(6)?;

    let dom = task.append_primal_exp_cone_domain()?;
    // (s0,1,x+y+log(2/Awall)) in PEXP
    task.put_afe_f_row(0,&[s[0]], &[1.0])?;
    task.put_afe_g(1,1.0)?;
    task.put_afe_f_row(2,&[xx[0],xx[1]], &[1.0,1.0])?; task.put_afe_g(2,(2.0/Aw).ln())?;
    task.append_acc(dom,&[0,1,2],&[0.0,0.0,0.0])?;

    // (s1,1,x+z+log(2/Awall)) in PEXP
    task.put_afe_f_row(3,&[s[1]], &[1.0])?;
    task.put_afe_g(4,1.0)?;
    task.put_afe_f_row(5,&[xx[0],xx[2]], &[1.0,1.0])?; task.put_afe_g(5,(2.0/Aw).ln())?;
    task.append_acc(dom,&[3,4,5],&[0.0,0.0,0.0])?;

    // s0+s1 < 1 <=> log(s0+s1) < 0
    task.put_a_row(0,s,&[1.0,1.0])?; task.put_con_bound(0, mosek::MSK_BK_UP, 1.0,1.0)?;

    task.put_a_row(1,&[xx[1],xx[2]], &[1.0,1.0])?; task.put_con_bound(1,mosek::MSK_BK_UP,Af.ln(),Af.ln())?;
    task.put_a_row(2,&[xx[0],xx[1]], &[1.0,-1.0])?; task.put_con_bound(1,mosek::MSK_BK_RA,alpha.ln(),beta.ln())?;
    task.put_a_row(3,&[xx[1],xx[2]], &[1.0,-1.0])?; task.put_con_bound(1,mosek::MSK_BK_RA,gamma.ln(),delta.ln())?;

    let _trm = task.optimize()?;
    let mut xyz = vec![0.0; 3];
    task.get_xx_slice(mosek::MSK_SOL_ITR, 0i32, numvar, xyz.as_mut_slice());

    task.write_data("gp1.ptf")?;

    Ok(xyz.iter().map(|v| v.exp()).collect())
}

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

    let  Aw    = 200.0;
    let Af    = 50.0;
    let alpha = 2.0;
    let beta  = 10.0;
    let gamma = 2.0;
    let delta = 10.0;

    let hwd = max_volume_box(Aw, Af, alpha, beta, gamma, delta)?;

    println!("h={:.4} w={:.4} d={:.4}\n", hwd[0], hwd[1], hwd[2]);

    Ok(())
}
