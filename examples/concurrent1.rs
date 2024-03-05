//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : concurrent1.rs
//!
//!  Purpose: Demonstrates a simple implementation of a concurrent optimizer.
//!
//!           The concurrent optimizer starts a few parallel optimizations
//!           of the same problem using different algorithms, and reports
//!           a solution when the first optimizer is ready.
//!
//!           This example also demonstrates how to define a simple callback handler
//!           that stops the optimizer when requested.

extern crate mosek;

use mosek::{Task,Objsense,Streamtype,Solsta,Soltype};
use std::sync::{Arc,Mutex};
use std::cmp::Ordering;
use std::thread;
use std::env;

fn optimize(mut t : mosek::Task, stop : Arc<Mutex<bool>>) -> Option<(i32,mosek::Task)> {
    let cbstop = Arc::clone(&stop);
    if let Some(trm) = t.with_callback(
        &mut |_| cbstop.lock().and_then(|p| Ok(*p)).unwrap_or(false),
        |task|
            if let Ok(trm) = task.optimize() {
                let mut st = stop.lock().unwrap();
                *st = true;
                Some(trm)
            } else {
                None
            }) {
        Some((trm,t))
    }
    else {
        None
    }
}

fn optimize_concurrent(task       : &mut mosek::Task,
                       optimizers : &[i32]) -> Vec<(usize,i32,mosek::Task)> {
    let stop = Arc::new(Mutex::new(false));
    optimizers.iter().enumerate()
        .filter_map(|(i,&ot)|
                    if let Some(mut t) = task.clone() {
                        if let Err(_) = t.put_int_param(mosek::Iparam::OPTIMIZER, ot as i32) { None }
                        else {
                            let stopopt = Arc::clone(&stop);
                            Some((i,thread::spawn(move || optimize(t,stopopt))))
                        }
                    }
                    else { None })
        .filter_map(|(i,th)|
                    match th.join().unwrap() {
                        None => None,
                        Some((r,t)) => Some((i,r,t)) } )
        .collect()
}

fn optimize_concurrent_mio(task  : & mut mosek::Task,
                           seeds : &[i32]) -> Vec<(usize,i32,mosek::Task)> {
    let stop = Arc::new(Mutex::new(false));

    seeds.iter().enumerate()
        .filter_map(|(i,&seed)| {
            if let Some(mut t) = task.clone() {
                if let Err(_) = t.put_int_param(mosek::Iparam::MIO_SEED, seed) { None }
                else {
                    let stopopt = Arc::clone(&stop);
                    Some((i,thread::spawn(move || optimize(t,stopopt))))
                }
            }
            else { None }})
        .filter_map(|(i,th)|
                    match th.join().unwrap() {
                        None => None,
                        Some((r,t)) => Some((i,r,t)) } )
        .collect()
}

enum FileOrText {
    File(String),
    Text(String)
}
fn main() -> Result<(),String> {
    let mut args = env::args();
    if args.len() < 2 {
        println!("Syntax: concurrent1 FILENAME [ TIMELIMIT ]");
        return Err("Invalid argument list".to_string());
    }
    let _ = args.next();
    let filename = args.next().unwrap();
    let timelimit = args.next();
    concurrent1(FileOrText::File(filename),
                timelimit)
}

fn concurrent1(data : FileOrText, timelimit : Option<String>) -> Result<(),String> {
    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        };

    match data {
        FileOrText::File(fname) => task.read_data(fname.as_str())?,
        FileOrText::Text(text)  => task.read_ptf_string(text.as_str())?
    }
    if let Some(timelimit) = timelimit {
        task.put_dou_param(mosek::Dparam::OPTIMIZER_MAX_TIME, timelimit.parse().unwrap())?;
    }

    let numintvar = task.get_num_int_var()?;

    let r = if numintvar == 0 {
        let optimizers = &[mosek::Optimizertype::CONIC,
                           mosek::Optimizertype::DUAL_SIMPLEX,
                           mosek::Optimizertype::PRIMAL_SIMPLEX];
        optimize_concurrent(& mut task, optimizers)
    }
    else {
        let seeds = &[ 42, 13, 71749373 ];
        optimize_concurrent_mio(& mut task, seeds)
    };


    let sense = task.get_obj_sense()?;
    // Pick the feasible result. For non-integer problems all
    // solutions should be the same if more than one is returned, but
    // for integer problems tasks may have hit the time limit and
    // returned non-optimal solutions.

    let n = r.len();

    if n == 0 {
        println!("All optimizers failed.");
    }
    else if numintvar > 0 {
        let (ii,_,tasks) = split3vec(r);

        let pobjs : Vec<(usize,f64)> =
            ii.iter().zip(tasks.iter()).enumerate()
            .filter_map(|(k,(_i,t))|
                match (*t).solution_def(Soltype::ITG) {
                    Ok(true) => match t.get_sol_sta(Soltype::ITG).unwrap() {
                        Solsta::PRIM_FEAS|Solsta::INTEGER_OPTIMAL => Some((k,t.get_primal_obj(Soltype::ITG).unwrap())),
                        _ => None
                    },
                    _ => None
                })
            .collect();

        let &(besti,bestobj) = pobjs.iter()
            .max_by(|(_,o1),(_,o2)|
                    match sense {
                        Objsense::MAXIMIZE => if o1 < o2 {Ordering::Less} else if o2 < o1 {Ordering::Greater} else {Ordering::Equal},
                        _ => if o1 > o2 {Ordering::Less} else if o2 > o1 {Ordering::Greater} else {Ordering::Equal}
                    }).unwrap();

        drop_except(tasks,besti).unwrap()
            .with_stream_callback(
                Streamtype::LOG, 
                &mut |msg| print!("{}",msg),
                |t| { 
                    t.optimizer_summary(mosek::Streamtype::LOG)?;
                    t.solution_summary(mosek::Streamtype::LOG)?;
                    Ok::<(),String>(())
                })?;

        println!("{} optimizers succeeded:",pobjs.len());
        for &(k,v) in pobjs.iter() {
            println!("Optimizer with seed #{} produced result : {:.5e}",ii[k],v);
        }

        println!("\tBest solution is #{}: {:.5e}",ii[besti],bestobj);
    }
    else {
        let (ii,_,tasks) = split3vec(r);

        let pobjs : Vec<(usize,f64)> =
            tasks.iter().enumerate()
            .filter_map(|(k,t)|
                        match t.get_sol_sta(Soltype::BAS) {
                            Ok(Solsta::PRIM_FEAS)|Ok(Solsta::OPTIMAL) => Some((k,t.get_primal_obj(Soltype::BAS).unwrap())),
                            _ => None
                        }.or_else(|| match t.get_sol_sta(Soltype::ITR) {
                            Ok(Solsta::PRIM_FEAS)|Ok(Solsta::OPTIMAL) => Some((k,t.get_primal_obj(Soltype::ITR).unwrap())),
                            _ => None
                        }))
            .collect();

        let &(besti,bestobj) = pobjs.iter()
            .max_by(|(_,o1),(_,o2)|
                    match sense {
                        Objsense::MAXIMIZE => if o1 < o2 {Ordering::Less} else if o2 < o1 {Ordering::Greater} else {Ordering::Equal},
                        _ => if o1 > o2 {Ordering::Less} else if o2 > o1 {Ordering::Greater} else {Ordering::Equal}
                    }).unwrap();

        drop_except(tasks,besti).unwrap()
            .with_stream_callback(
                Streamtype::LOG, 
                &mut|msg| print!("{}",msg),
                |t| {
                    t.optimizer_summary(mosek::Streamtype::LOG)?;
                    t.solution_summary(mosek::Streamtype::LOG)?;
                    Ok::<(),String>(())
                })?;

        println!("{} optimizers succeeded:",pobjs.len());
        for &(k,v) in pobjs.iter() {
            println!("Optimizer with seed #{} produced result : {:.5e}",ii[k],v);
        }

        println!("\tBest solution is #{}: {:.5e}",ii[besti],bestobj);
    }


    Result::Ok(())
}


fn drop_except<A>(mut a : Vec<A>, idx : usize ) -> Option<A> {
    if idx >= a.len() { None }
    else {
        for _i in 0..a.len()-idx-1 { let _ = a.pop(); }
        a.pop()
    }
}

fn split3vec<A,B,C>(mut v : Vec<(A,B,C)>) -> (Vec<A>,Vec<B>,Vec<C>) {
    let mut ra = Vec::with_capacity(v.len());
    let mut rb = Vec::with_capacity(v.len());
    let mut rc = Vec::with_capacity(v.len());

    while let Some((va,vb,vc)) = v.pop() {
        ra.push(va);
        rb.push(vb);
        rc.push(vc);
    }

    ra.reverse();
    rb.reverse();
    rc.reverse();
    (ra,rb,rc)
}


#[cfg(test)]
mod tests {
    const DFLT_FILE1 : &str = "Task
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

    const DFLT_FILE2 : &str = "Task
Objective
    Maximize + @x0 + 0.64 @x1
Constraints
    @c0 [-inf;250] + 50 @x0 + 31 @x1
    @c1 [-4;+inf] + 3 @x0 - 2 @x1
Variables
    @x0[0;+inf]
    @x1[0;+inf]
Integers
    @x0 @x1
";
    #[test]
    fn test() {
        super::concurrent1(super::FileOrText::Text(DFLT_FILE1.to_string()),None).unwrap();
        super::concurrent1(super::FileOrText::Text(DFLT_FILE2.to_string()),Some("100.0".to_string())).unwrap();
    }
}
