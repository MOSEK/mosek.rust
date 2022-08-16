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

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Missing argument, syntax is:");
        println!("  opt_server_async inputfile http[s]://HOSTNAME:PORT [certfile]");
        Err("Missing arguments".to_string())
    }
    else {
        let inputfile = args[1].as_str();
        let addr      = args[2].as_str();
        let cert      = if args.len() < 4 { None } else { Some(args[3].as_str()) };

        let mut task = Task::new().unwrap().with_callbacks();
        task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

        // Load some data into the task
        task.read_data(inputfile)?;

        // Set OptServer URL
        task.put_optserver_host(addr)?;

        // Path to certificate, if any
        if let Some(cert) = cert {
            task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH, cert)?;
        }

        // Optimize remotely, no access token
        let _trm = task.optimize()?;

        task.solution_summary(Streamtype::LOG)?;

        Ok(())
    }
}
