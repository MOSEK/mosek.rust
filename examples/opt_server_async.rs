//!
//!   Copyright : ==COPYRIGHT==
//!
//!   File : ==FILE==
//!
//!   Purpose :   Demonstrates how to use MOSEK OptServer
//!               to solve optimization problem asynchronously

//TAG:begin-code
extern crate mosek;
extern crate itertools;

use mosek::{Task,Streamtype,Sparam};
use std::env;
use std::time::Duration;
use std::thread::sleep;
use itertools::{Either,Either::*};

fn main() {
    let mut args = env::args();
    if args.len() < 3 {
        println!("Missing argument, syntax is:");
        println!("  opt_server_async inputfile http[s]://HOSTNAME:PORT numpolls [certfile]");
        panic!("Missing arguments");
    }
    let _ = args.next();
    opt_server_async(Either::Right(args.next().unwrap()),
                     args.next().unwrap(),
                     args.next().unwrap().parse().unwrap(),
                     args.next()).unwrap();
}
fn opt_server_async(inputfile : Either<String,String>, addr : String, numpolls : usize, cert : Option<String>) -> Result<(),String> {
    // Path to certificate, if any

    let token = {
        Task::new().unwrap()
            .with_stream_callback(
                Streamtype::LOG,
                &mut |msg| print!("{}",msg),
                |task| {
                    match &inputfile {
                        Either::Right(filename) => task.read_data(filename.as_str()).unwrap(),
                        Either::Left(data)     => task.read_ptf_string(data.as_str()).unwrap()
                    }
                    if let Some(ref cert) = cert {
                        task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH,cert.as_str())?;
                    }
                    task.async_optimize(addr.as_str(),"")
                }).expect("Failed to submit async optimization")
    };

    println!("Task token = '{}'", token);


    println!("Setting log stream...");
    Task::new().unwrap().with_stream_callback(
        Streamtype::LOG,
        & mut |msg| print!("{}",msg),
        |task| task.with_callback(
            &mut|caller| { println!("caller = {}",caller); false },
            |task| {
                println!("Reading input file '{:?}'...",inputfile);
                match &inputfile {
                    Right(filename) => task.read_data(filename.as_str()).unwrap(),
                    Left(data)     => task.read_ptf_string(data.as_str()).unwrap()
                }
                if let Some(ref cert) = cert {
                    task.put_str_param(Sparam::REMOTE_TLS_CERT_PATH,cert.as_str())?;
                }

                println!("Starting polling loop...");
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
            }))
}
//TAG:end-code



#[cfg(test)]
mod tests {
    use itertools::Either;
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
        super::opt_server_async(Either::Left(DFLT_FILE.to_string()),
                                "http://solve.mosek.com:30080".to_string(),
                                100,
                                None).unwrap();
    }
}
