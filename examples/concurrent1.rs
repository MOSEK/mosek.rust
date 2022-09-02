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

fn optimize(t : mosek::Task, stop : Arc<Mutex<bool>>) -> Option<(i32,mosek::Task)> {
    let mut t = t.with_callbacks();
    let cbstop = Arc::clone(&stop);
    if let Err(_) = t.put_callback(move |_,_,_,_| ! *(cbstop.lock().unwrap()) ) { None }
    else if let Ok(trm) = t.optimize() {
        let mut st = stop.lock().unwrap();
        *st = true;
        Some((trm,t.without_callbacks()))
    }
    else { None }
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

fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Syntax: concurrent1 FILENAME [ TIMELIMIT ]");
        return Err("Invalid argument list".to_string());
    }

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
        };

    task.read_data(args[1].as_str())?;
    if args.len() > 2 {
        task.put_dou_param(mosek::Dparam::OPTIMIZER_MAX_TIME, args[2].parse().unwrap())?;
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

        {
            let mut t = drop_except(tasks,besti).unwrap().with_callbacks();

            t.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
            t.optimizer_summary(mosek::Streamtype::LOG)?;
            t.solution_summary(mosek::Streamtype::LOG)?;
        }

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
                        }) )
            .collect();

        let &(besti,bestobj) = pobjs.iter()
            .max_by(|(_,o1),(_,o2)|
                    match sense {
                        Objsense::MAXIMIZE => if o1 < o2 {Ordering::Less} else if o2 < o1 {Ordering::Greater} else {Ordering::Equal},
                        _ => if o1 > o2 {Ordering::Less} else if o2 > o1 {Ordering::Greater} else {Ordering::Equal}
                    }).unwrap();

        {
            let mut t = drop_except(tasks,besti).unwrap().with_callbacks();

            t.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
            t.optimizer_summary(mosek::Streamtype::LOG)?;
            t.solution_summary(mosek::Streamtype::LOG)?;
        }

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
    #[test]
    fn test_concurrent1() {
        super::main().unwrap();
    }
}
