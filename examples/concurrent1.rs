//!  Copyright: MOSEK ApS
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

fn optimize(t : Arc<Mutex<mosek::Task>>, stop : Arc<Mutex<bool>>) -> Option<i32> {
    let t = t.lock().unwrap();
    if let Ok(trm) = (*t).optimize() {
        let mut st = stop.lock().unwrap();
        *st = true;
        Some(trm)
    }
    else { None }
}

fn optimize_concurrent(task       : &mut mosek::Task,
                       optimizers : &[mosek::Optimizertype]) -> Vec<(usize,i32,Arc<Mutex<mosek::Task>>)> {
    let flag = Arc::new(Mutex::new(false));


    optimizers.iter().enumerate()
        .filter_map(|(i,&ot)|
                    if let Ok(t) = task.clone_task() {
                        let stop = Arc::clone(&flag);
                        if      let Err(_) = t.put_callback(move |_,_,_,_| ! *stop.lock().unwrap() ) { None }
                        else if let Err(_) = t.put_int_param(mosek::Iparam::OPTIMIZER, ot as i32) { None }
                        else { Some((i,Arc::new(Mutex::new(t)))) }
                    }
                    else { None })
        .map(|(i,t)| (i,t,thread::spawn(move || optimize(Arc::clone(&t),Arc::clone(&flag)))))
        .filter_map(|(i,t,br)|
                    match br.join().unwrap() {
                        None => None,
                        Some(r) => Some((i,r,t)) } )
        .collect()
}

fn optimize_concurrent_mio(task  : & mut mosek::Task,
                           seeds : &[i32]) -> Vec<(usize,i32,Arc<Mutex<mosek::Task>>)> {
    let flag = Arc::new(Mutex::new(false));

    seeds.iter().enumerate()
        .filter_map(|(i,&seed)| {
            if let Ok(t) = task.clone_task() {
                let stop = Arc::clone(&flag);
                if      let Err(_) = t.put_callback(move |_,_,_,_| { let s = stop.lock().unwrap(); ! *s }) { None }
                else if let Err(_) = t.put_int_param(mosek::Iparam::MIO_SEED, seed) { None }
                else { Some((i,Arc::new(Mutex::new(t)))) }
            }
            else { None }
        })
        .map(|(i,t)| (i,t,thread::spawn(move || optimize(Arc::clone(&t),Arc::clone(&flag)))))
        .filter_map(|(i,t,br)|
                    match br.join().unwrap() {
                        None => None,
                        Some(r) => Some((i,t,r)) } )
        .collect()
}

    // f (firstOK >= 0)
    // {
    //   // Pick the task that ended with res = ok
    //   // and contains an integer solution with best objective value
    //   mosek.objsense sense = task.getobjsense();
    //   double bestObj = (sense == mosek.objsense.minimize) ? 1.0e+10 : -1.0e+10;
    //   int bestPos = -1;

    //   for (int i = 0; i < n; ++i)
    //     System.out.println(i + "    " + tasks[i].getprimalobj(mosek.soltype.itg));

    //   for (int i = 0; i < n; ++i)
    //     if ((res[i] == mosek.rescode.ok) &&
    //         (tasks[i].getsolsta(mosek.soltype.itg) == mosek.solsta.prim_feas ||
    //          tasks[i].getsolsta(mosek.soltype.itg) == mosek.solsta.integer_optimal) &&
    //         ((sense == mosek.objsense.minimize) ? 
    //             (tasks[i].getprimalobj(mosek.soltype.itg) < bestObj) :
    //             (tasks[i].getprimalobj(mosek.soltype.itg) > bestObj)   )   )
    //     {
    //       bestObj = tasks[i].getprimalobj(mosek.soltype.itg);
    //       bestPos = i;
    //     }

    //   if (bestPos != -1)
    //   {
    //     winTask[0]  = tasks[bestPos]; 
    //     winTrm[0]   = trm[bestPos]; 
    //     winRes[0]   = res[bestPos];
    //     return bestPos;
    //   }
    // }
  
    // return -1;
//u  }


fn main() -> Result<(),String> {
    let args: Vec<String> = env::args().collect();

    println!("args = {:?}",args);
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
        optimize_concurrent(& mut task, optimizers)?
    }
    else {
        let seeds = &[ 42, 13, 71749373 ];
        optimize_concurrent_mio(& mut task, seeds)?
    };


    let sense = task.get_obj_sense()?;
    // Pick the feasible result. For non-integer problems all
    // solutions should be the same if more than one is returned, but
    // for integer problems tasks may have hit the time limit and
    // returned non-optimal solutions.
    if r.len() == 0 {
        println!("All optimizers failed.");
    }
    else if numintvar > 0 {
        let sols = r.filter_map(|(i,(_trm,t))|
                                match t.solution_def(Soltype::ITG) {
                                    Err(_) => None,
                                    Ok(false) => None,
                                    _ => match t.get_sol_sta(Soltype::ITG).unwrap() {
                                        Solsta::PRIM_FEAS|Solsta::INTEGER_OPTIMAL => Some((i,t.get_primal_obj(Soltype::ITG).unwrap(),t)),
                                        _ => None
                                    }
                                }).collect();
        let (besti,bestobj) = sols
            .map(|(&i,&v,_)| (i,v))
            .max_by(|(_,&o1),(_,&o2)|
                    match sense {
                        Objsense::MAXIMIZE => if o1 < o2 {Ordering::Less} else if o2 < o1 {Ordering::Greater} else {Ordering::Equal},
                        Objsense::MINIMIZE => if o1 > o2 {Ordering::Less} else if o2 > o1 {Ordering::Greater} else {Ordering::Equal}
                    });
        {
            let (_,(_,& mut t)) = sols[besti];
            t.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
            t.optimizer_summary(mosek::Streamtype::LOG)?;
            t.solution_summary(mosek::Streamtype::LOG)?;
        }

        println!("{} optimizers succeeded:",r.len());
        for (&i,&v,_t) in sols {
            println!("Optimizer with seed #{} produced result : {:.5e}",i,v);
        }

        println!("\tBest solution is: {:.5e}",bestobj);
    }
    else {
        let sols = r.filter_map(|(i,(_trm,t))|
                                match t.solution_def(Soltype::BAS) {
                                    Ok(true) => match t.get_sol_sta(Soltype::BAS).unwrap() {
                                        Solsta::PRIM_FEAS|Solsta::OPTIMAL => Some((Soltype::BAS,i,t.get_primal_obj(Soltype::BAS).unwrap(),t)),
                                        _ => None
                                    }
                                }.or_else(||
                                          match t.solution_def(Soltype::ITR) {
                                              Ok(true) => match t.get_sol_sta(Soltype::ITR).unwrap() {
                                                  Solsta::PRIM_FEAS|Solsta::OPTIMAL => Some((Soltype::ITR,i,t.get_primal_obj(Soltype::ITR).unwrap(),t)),
                                                  _ => None
                                              }
                                          }
                                )).collect();
        let (besti,bestobj) = sols
            .map(|(&i,&v,_)| (i,v))
            .max_by(|(_,&o1),(_,&o2)|
                    match sense {
                        Objsense::MAXIMIZE => if o1 < o2 {Ordering::Less} else if o2 < o1 {Ordering::Greater} else {Ordering::Equal},
                        Objsense::MINIMIZE => if o1 > o2 {Ordering::Less} else if o2 > o1 {Ordering::Greater} else {Ordering::Equal}
                    });
        {
            let (_,(_,& mut t)) = sols[besti];
            t.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;
            t.optimizer_summary(mosek::Streamtype::LOG)?;
            t.solution_summary(mosek::Streamtype::LOG)?;
        }

        println!("{} optimizers succeeded:",r.len());
        for (&i,&v,_t) in sols {
            println!("Optimizer with seed #{} produced result : {:.5e}",i,v);
        }

        println!("\tBest solution is: {:.5e}",bestobj);
    }

















    
    return Result::Ok(());
}
