//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : simple.rs
//!
//!   Purpose :   Demonstrates a very simple example using MOSEK by
//!               reading a problem file, solving the problem and
//!               writing the problem+solution to a file.

extern crate mosek;
use mosek::{Task,Streamtype,Iparam,Onoffkey};
use std::env;

enum FileOrText {
    File(String),
    Text(String)
}
fn main() -> Result<(),String> {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Syntax: simple FILENAME [ OUTFILE ]");
        return Err("Invalid argument list".to_string())
    }
    let _ = args.next();
    simple(FileOrText::File(args.next().unwrap()),
           args.next())
}

fn simple(filename : FileOrText, outfile : Option<String>) -> Result<(),String> {
    let mut task = Task::new().unwrap().with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // We assume that a problem file was given as the first command
    // line argument (received in `args')
    match filename {
        FileOrText::File(fname) => {
            task.read_data(fname.as_str())?
        },
        FileOrText::Text(data) => {
            task.read_ptf_string(data.as_str())?
        }
    }
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

        task.write_data(outfile.as_str())?;
    }
    Ok(())
}




#[cfg(test)]
mod tests {

    const DFLT_FILE : &str = "Task
Objective
    Maximize + 2 @x0 + 3 @x1 - @x2
Constraints
    @c0 [1] + @x0 + @x1 + @x2
    @C0 [QUAD(3)]
        @ac1: + 0.03
        @ac2: + 1.5 @x0 + 0.1 @x1
        @ac3: + 0.3 @x0 + 2.1 @x2 + 0.1
Variables
    @x0
    @x1
    @x2
";

    #[test]
    fn test() {
        super::simple(super::FileOrText::Text(DFLT_FILE.to_string()),None).unwrap();
    }
}
