//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : opt_server_async.rs
//!
//!   Purpose :   Demonstrates how to use MOSEK OptServer
//!               to solve optimization problem asynchronously

extern crate mosek;

use mosek::{Task,Streamtype,Sparam};
use std::env;
use std::time::Duration;
use std::thread::sleep;

enum FileOrText {
    File(String),
    Text(String)
}
fn main() -> Result<(),String> {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Missing argument, syntax is:");
        println!("  opt_server_async inputfile http[s]://HOSTNAME:PORT numpolls [certfile]");
        return Err("Missing arguments".to_string())
    }
    let _ = args.next();
    opt_server_async(FileOrText::File(args.next().unwrap()),
                     args.next().unwrap(),
                     args.next().unwrap().parse().unwrap(),
                     args.next())
}
fn opt_server_async(inputfile : FileOrText, addr : String, numpolls : usize, cert : Option<String>) -> Result<(),String> {
    // Path to certificate, if any

    let token = {
        let mut task = Task::new().unwrap();
        match inputfile {
            FileOrText::File(ref filename) => task.read_data(filename.as_str())?,
            FileOrText::Text(ref data)     => task.read_ptf_string(data.as_str())?
        }
        if let Some(ref cert) = cert {
            task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH,cert.as_str())?;
        }
        task.async_optimize(addr.as_str(),"")?
    };

    println!("Task token = '{}'", token);

    {
        let mut task = Task::new().unwrap().with_callbacks();
        task.put_callback(|caller,_,_,_| { println!("caller = {}",caller); true })?;
        println!("Reading input file...");
        match inputfile {
            FileOrText::File(ref filename) => task.read_data(filename.as_str())?,
            FileOrText::Text(ref data)     => task.read_ptf_string(data.as_str())?
        }
        if let Some(ref cert) = cert {
            task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH,cert.as_str())?;
        }

        println!("Setting log stream...");
        task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

        println!("Starting polling loop...");
        //int i = 0;
        for i in 0..numpolls {
            sleep(Duration::new(1,0));

            println!("\tpoll {}...", i);

            let mut trm  : i32 = 0;
            let mut resp : i32 = 0;

            let respavailable = task.async_poll(addr.as_str(),
                                                "",
                                                token.as_str(),
                                                & mut resp,
                                                & mut trm)?;

            if respavailable {
                println!("solution available!");

                task.async_get_result(addr.as_str(),
                                      "",
                                      token.as_str(),
                                      & mut resp,
                                      & mut trm)?;

                task.solution_summary (Streamtype::LOG)?;
                return Ok(());
            }
        }

        println!("max num polls reached, stopping host.");
        task.async_stop (addr.as_str(), "", token.as_str())?;
        Err("Max num polls".to_string())
    }
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
        super::opt_server_async(super::FileOrText::Text(DFLT_FILE.to_string()),
                                "http://solve.mosek.com:30080".to_string(),
                                100,
                                None).unwrap();
    }
}
