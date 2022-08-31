//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : $${file}
//!
//!   Purpose :   Demonstrates how to use MOSEK OptServer
//!               to solve optimization problem asynchronously

/*TAG:begin-code*/
extern crate mosek;

use mosek::{Task,Streamtype,Sparam};
use std::env;
use std::time::Duration;
use std::thread::sleep;

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        println!("Missing argument, syntax is:");
        println!("  opt_server_async inputfile http[s]://HOSTNAME:PORT numpolls");
        Err("Missing arguments".to_string())
    }
    else {
        let inputfile = args[1].as_str();
        let addr      = args[2].as_str();
        
        let numpolls : i32  = args[3].as_str().parse().unwrap();
        let cert      = if args.len() < 5 { None } else { Some(args[4].as_str()) };

        let token = {
            let mut task = Task::new().unwrap();
            task.read_data(inputfile)?;
            if let Some(cert) = cert {
                task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH,cert)?;
            }
            task.async_optimize(addr,"")?
        };

        println!("Task token = '{}'", token);

        {
            let mut task = Task::new().unwrap().with_callbacks();
            task.put_callback(|caller,_,_,_| { println!("caller = {}",caller); true })?;
            println!("Reading input file...");
            task.read_data(inputfile)?;
            if let Some(cert) = cert {
                task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH,cert)?;
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

                let respavailable = task.async_poll(addr,
                                                    "",
                                                    token.as_str(),
                                                    & mut resp,
                                                    & mut trm)?;

                if respavailable {
                    println!("solution available!");

                    task.async_get_result(addr,
                                          "",
                                          token.as_str(),
                                          & mut resp,
                                          & mut trm)?;

                    task.solution_summary (Streamtype::LOG)?;
                    return Ok(());
                }
            }

            println!("max num polls reached, stopping host.");
            task.async_stop (addr, "", token.as_str())?;
            Err("Max num polls".to_string())
        }
    }
}
/*TAG:end-code*/
