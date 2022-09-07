//! Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//! File : writecallback.rs
//!
//! Purpose: Show how to use callback file writer
//!

extern crate mosek;

use mosek::{Task,Dataformat,Compresstype};
use std::io::{Write,stdout};

const DFLT_FILE : &str = "Task
Objective
    Maximize + 3 @x0 + @x1 + 5 @x2 + @x3
Constraints
    @c0 [30] + 3 @x0 + @x1 + 2 @x2
    @c1 [15;+inf] + 2 @x0 + @x1 + 3 @x2 + @x3
    @c2 [-inf;25] + 2 @x1 + 3 @x3
Variables
    @x0 [0;+inf]
    @x1 [0;10]
    @x2 [0;+inf]
    @x3 [0;+inf]
";

fn main() -> Result<(),String> {
    let mut task = Task::new().unwrap();
    task.read_ptf_string(DFLT_FILE).unwrap();

    task.write_data_stream(|s| if let Err(_) = stdout().write_all(s) { 0 } else { s.len() },
                           Dataformat::PTF,
                           Compresstype::NONE)?;
    Ok(())
}

#[cfg(test)]
mod tests {
            #[test]
    fn test() {
        super::main().unwrap();
    }
}
