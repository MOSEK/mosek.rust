//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : callback.rs
//!
//!  Purpose :   To demonstrate how to use the progress
//!              callback.
//!
//!              Use this script as follows:
//!
//!               callback psim  25fv47.mps
//!               callback dsim  25fv47.mps
//!               callback intpnt 25fv47.mps
//!
//!              The first argument tells which optimizer to use
//!              i.e. psim is primal simplex, dsim is dual simplex
//!              and intpnt is interior-point.


extern crate mosek;

use std::env;
use mosek::{Task,Streamtype,Iparam,Optimizertype,Callbackcode,Dinfitem,Iinfitem};

const MAXTIME : f64 = 0.05;

fn callback(caller : i32, dinf : &[f64], iinf : &[i32], _linf : &[i64]) -> bool {
    let mut opttime = 0.0;
    match caller {
        Callbackcode::BEGIN_INTPNT =>
            println!("Starting interior-point optimizer"),
        Callbackcode::INTPNT => {
            let itrn  = iinf[Iinfitem::INTPNT_ITER as usize];
            let pobj  = dinf[Dinfitem::INTPNT_PRIMAL_OBJ as usize];
            let dobj  = dinf[Dinfitem::INTPNT_DUAL_OBJ as usize];
            let stime = dinf[Dinfitem::INTPNT_TIME as usize];
            opttime   = dinf[Dinfitem::OPTIMIZER_TIME as usize];

            println!("Iterations: {:-3}",itrn);
            println!("  Elapsed time: {:6.2}({:.2})",opttime, stime);
            println!("  Primal obj.: {:-18.6e}  Dual obj.: {:-18.6e}",pobj, dobj);
        },
        Callbackcode::END_INTPNT => 
            println!("Interior-point optimizer finished."),
        Callbackcode::BEGIN_PRIMAL_SIMPLEX =>
            println!("Primal simplex optimizer started."),
        Callbackcode::UPDATE_PRIMAL_SIMPLEX => {
            let itrn  = iinf[Iinfitem::SIM_PRIMAL_ITER as usize];
            let pobj  = dinf[Dinfitem::SIM_OBJ as usize];
            let stime = dinf[Dinfitem::SIM_TIME as usize];
            opttime   = dinf[Dinfitem::OPTIMIZER_TIME as usize];
            println!("Iterations: {:-3}",itrn);
            println!("  Elapsed time: {:6.2}({:.2})",opttime, stime);
            println!("  Obj.: {:-18.6e}",pobj);
        },
        Callbackcode::END_PRIMAL_SIMPLEX =>
            println!("Primal simplex optimizer finished."),
        Callbackcode::BEGIN_DUAL_SIMPLEX =>
            println!("Dual simplex optimizer started."),
        Callbackcode::UPDATE_DUAL_SIMPLEX => {
            let itrn  = iinf[Iinfitem::SIM_DUAL_ITER as usize];
            let pobj  = dinf[Dinfitem::SIM_OBJ as usize];
            let stime = dinf[Dinfitem::SIM_TIME as usize];
            opttime   = dinf[Dinfitem::OPTIMIZER_TIME as usize];
            println!("Iterations: {:-3}", itrn);
            println!("  Elapsed time: {:6.2}({:.2})",opttime,stime);
            println!("  Obj.: {:-18.6e}",pobj);
        },
        Callbackcode::END_DUAL_SIMPLEX =>
            println!("Dual simplex optimizer finished."),
        Callbackcode::NEW_INT_MIO => {
            println!("New integer solution has been located.");
            // let mut xx = vec![0.0; ]
            // xx = task.get_xx(Soltype::ITG);
            // println!(xx);
            println!("Obj.: {}",dinf[Dinfitem::MIO_OBJ_INT as usize]);
        }
        _ => {
        }
    }

    if opttime >= MAXTIME {
        // mosek is spending too much time. Terminate it.
        println!("Terminating.");
        false
    }
    else {
        true
    }
}


fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Syntax: callback (psim|dsim|intpnt) FILENAME");
        Err("Invalid argument list".to_string())
    }
    else {

        /* Create the optimization task. */
        let mut task = Task::new().unwrap().with_callbacks();
        task.read_data(args[2].as_str())?;

        match args[1].as_str() {
            "psim"   => task.put_int_param(Iparam::OPTIMIZER,Optimizertype::PRIMAL_SIMPLEX)?,
            "dsim"   => task.put_int_param(Iparam::OPTIMIZER,Optimizertype::DUAL_SIMPLEX)?,
            "intpnt" => task.put_int_param(Iparam::OPTIMIZER,Optimizertype::INTPNT)?,
            s => return Err(format!("Invalid argument '{}'",s))
        }

        /* Directs the log task stream to the 'printstr' function. */
        task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
        task.put_callback(callback)?;

        task.optimize()?;

        Result::Ok(())
    }
}
