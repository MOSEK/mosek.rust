//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : opt_server_sync.rs
//!
//!   Purpose :   Demonstrates how to use MOSEK OptServer
//!               to solve optimization problem synchronously
//!
extern crate mosek;

use mosek::{Task,Streamtype,Sparam};
use std::env;

enum FileOrText {
    File(String),
    Text(String)
}
fn main() -> Result<(),String> {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Missing argument, syntax is:");
        println!("  opt_server_sync inputfile http[s]://HOSTNAME:PORT [certfile]");
        return Err("Missing arguments".to_string())
    }
    let _ = args.next();
    opt_server_sync(FileOrText::File(args.next().unwrap()),
                    args.next().unwrap(),
                    args.next())
}

fn opt_server_sync(inputfile : FileOrText, addr : String, cert : Option<String>) -> Result<(),String> {
    let mut task = Task::new().unwrap().with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    // Load some data into the task
    match inputfile {
        FileOrText::File(filename) => task.read_data(filename.as_str())?,
        FileOrText::Text(data) => task.read_ptf_string(data.as_str())?
    }

    // Set OptServer URL
    task.put_optserver_host(addr.as_str())?;

    // Path to certificate, if any
    if let Some(cert) = cert {
        task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH, cert.as_str())?;
    }

    // Optimize remotely, no access token
    let _trm = task.optimize()?;

    task.solution_summary(Streamtype::LOG)?;

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
        super::opt_server_sync(super::FileOrText::Text(DFLT_FILE.to_string()),
                               "http://solve.mosek.com:30080".to_string(),
                               None).unwrap();
    }
}
