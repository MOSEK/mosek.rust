//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : simple.rs
//!
//!   Purpose :   Demonstrates a very simple example using MOSEK by
//!               reading a problem file, solving the problem and
//!               writing the problem+solution to a file.

/*TAG:begin-code*/
extern crate mosek;
use mosek::{Task,Streamtype,Iparam,Onoffkey};
use std::env;

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Syntax: simple FILENAME [ OUTFILE ]");
        Err("Invalid argument list".to_string())
    }
    else {
        let filename = args[0].as_str();
        let outfile = if args.len() > 2 { Some(args[1].as_str()) } else { None };

        let mut task = Task::new().unwrap().with_callbacks();
        task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

        // We assume that a problem file was given as the first command
        // line argument (received in `args')
        /*TAG:begin-readdata*/
        task.read_data (filename)?;
        /*TAG:end-readdata*/

        // Solve the problem
            let _ = task.optimize()?;

        // Print a summary of the solution
        task.solution_summary(Streamtype::LOG)?;

        // If an output file was specified, save problem to file
        if let Some(outfile) = outfile {
            // If using OPF format, these parameters will specify what to include in output
            task.put_int_param(Iparam::OPF_WRITE_SOLUTIONS,  Onoffkey::ON)?;
            task.put_int_param(Iparam::OPF_WRITE_PROBLEM,    Onoffkey::ON)?;
            task.put_int_param(Iparam::OPF_WRITE_HINTS,      Onoffkey::OFF)?;
            task.put_int_param(Iparam::OPF_WRITE_PARAMETERS, Onoffkey::OFF)?;

            task.put_int_param(Iparam::PTF_WRITE_SOLUTIONS,  Onoffkey::ON)?;

            /*TAG:begin-writedata*/
            task.write_data(outfile)?;
            /*TAG:end-writedata*/
        }
        Ok(())
    }
}
/*TAG:end-code*/
