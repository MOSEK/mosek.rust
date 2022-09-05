//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   File : mioinitsol.rs
//!
//!   Purpose :   Demonstrates how to solve a MIP with a start guess.
//!

extern crate mosek;

use mosek::{Task,Boundkey,Variabletype,Objsense,Streamtype,Soltype};

const INF : f64 = 0.0;

fn main() -> Result<(),String> {
    let numvar : i32 = 4;
    let numcon : i32 = 1;

    let c = &[ 7.0, 10.0, 1.0, 5.0 ];

    let bkc = &[Boundkey::UP];
    let blc = &[ -INF ];
    let buc = &[2.5];
    let bkx = &[Boundkey::LO,
                Boundkey::LO,
                Boundkey::LO,
                Boundkey::LO ];
    let blx = &[0.0,
                0.0,
                0.0,
                0.0 ];
    let bux = &[INF,
                INF,
                INF,
                INF];

    let ptrb    = &[0i64, 1, 2, 3];
    let ptre    = &[1i64, 2, 3, 4];
    let aval    = &[1.0, 1.0, 1.0, 1.0];
    let asub    = &[0i32,   0,   0,   0  ];
    let intsub  = &[0i32, 1, 2];
    let inttype = &[Variabletype::TYPE_INT,
                    Variabletype::TYPE_INT,
                    Variabletype::TYPE_INT ];

    let mut task = Task::new().unwrap().with_callbacks();
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    task.input_data(numcon, numvar,
                    c, 0.0,
                    ptrb, ptre,
                    asub, aval,
                    bkc, blc, buc,
                    bkx, blx, bux)?;

    task.put_var_type_list(intsub, inttype)?;

    /* A maximization problem */
    task.put_obj_sense(Objsense::MAXIMIZE)?;

    // Assign values to integer variables
    // We only set that slice of xx
    task.put_xx_slice(Soltype::ITG, 0, 3, &[1.0,1.0,0.0])?;

    // Request constructing the solution from integer variable values
    task.put_int_param(mosek::Iparam::MIO_CONSTRUCT_SOL, mosek::Onoffkey::ON)?;

    // solve
    let _ = task.optimize()?;
    task.solution_summary(mosek::Streamtype::LOG)?;

    // Read and print solution
    let mut xx = vec![0.0; numvar as usize];
    task.get_xx(mosek::Soltype::ITG, xx.as_mut_slice())?;
    println!("Optimal solution:");
    for (i,xi) in xx.iter().enumerate() {
        println!("x[{}] = {}",i,*xi);
    }

    // Was the initial solution used?
    let constr = task.get_int_inf(mosek::Iinfitem::MIO_CONSTRUCT_SOLUTION)?;
    let constr_val = task.get_dou_inf(mosek::Dinfitem::MIO_CONSTRUCT_SOLUTION_OBJ)?;
    println!("Construct solution utilization: {}", constr);
    println!("Construct solution objective: {}",  constr_val);
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
