/*
   Copyright: $$copyright

   File:    $${file}

   Purpose:  Demonstrates how to solve a small mixed
             integer linear optimization problem using the MOSEK C# API.
 */

extern crate mosek;

fn main() -> Result<(),String> {
    let numcon = 2;
    let numvar = 2;

    // Since the value infinity is never used, we define
    // 'infinity' symbolic purposes only

    let bkc = vec![mosek::MSK_BK_UP, mosek::MSK_BK_LO];
    let blc = vec![-f64::INFINITY, -4.0 ];
    let buc = vec![250.0, f64::INFINITY ];

    let bkx = vec![mosek::MSK_BK_LO, mosek::MSK_BK_LO];
    let blx = vec![0.0, 0.0];
    let bux = vec![f64::INFINITY, f64::INFINITY];

    let c   = vec![1.0, 0.64];
    let aptr = vec![0,2,4];
    let asub = vec![0,1,
                    0,1];
    let aval = vec![50.0,3.0,31.0,-2.0];


    let env = match mosek::Env::new() {
        Some(e) => e,
        None => return Err("Failed to create env".to_string()) };
    let mut task = match env.task() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()) };

    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;
    task.put_callback(|caller,_,_,_| { println!("caller = {}",caller); true })?;

    /* Append 'numcon' empty constraints.
    The constraints will initially have no bounds. */
    task.append_cons(numcon as i32)?;

    /* Append 'numvar' variables.
    The variables will initially be fixed at zero (x=0). */
    task.append_vars(numvar as i32)?;

    /* Optionally add a constant term to the objective. */
    task.put_cfix(0.0)?;

    for j in 0..numvar {
        /* Set the linear term c_j in the objective.*/
        task.put_c_j(j as i32, c[j])?;
        /* Set the bounds on variable j.
        blx[j] <= x_j <= bux[j] */
        task.put_var_bound(j as i32, bkx[j], blx[j], bux[j])?;
        /* Input column j of A */
        task.put_a_col(j as i32,              /* Variable (column) index.*/
                       &asub[aptr[j]..aptr[j+1]],               /* Row index of non-zeros in column j.*/
                       &aval[aptr[j]..aptr[j+1]])?;              /* Non-zero Values of column j. */
    }
    /* Set the bounds on constraints.
    for i=1, ...,numcon : blc[i] <= constraint i <= buc[i] */
    for i in 0..numcon {
        task.put_con_bound(i as i32, bkc[i], blc[i], buc[i])?;
    }


    /* Specify integer variables. */
    for j in 0..numvar {
        task.put_var_type(j as i32, mosek::MSK_VAR_TYPE_INT)?;
    }
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE)?;

    /* Set max solution time */
    task.put_dou_param(mosek::MSK_DPAR_MIO_MAX_TIME, 60.0)?;

    task.write_data("milo1.ptf")?;
    task.optimize()?;

    // Print a summary containing information
    //   about the solution for debugging purposes
    task.solution_summary(mosek::MSK_STREAM_MSG)?;

    /* Get status information about the solution */
    let solsta = task.get_sol_sta(mosek::MSK_SOL_ITG)?;
    let mut xx : Vec<f64> = Vec::new(); xx.resize(numvar,0.0);
    task.get_xx(mosek::MSK_SOL_ITG, // Integer solution.
               xx.as_mut_slice())?;

    match solsta {
        mosek::MSK_SOL_STA_OPTIMAL | mosek::MSK_SOL_STA_INTEGER_OPTIMAL => {
            println!("Optimal primal solution");
            for j in 0..numvar {
                println!("x[{}]: {:.4}", j, xx[j]);
            }
        },
        mosek::MSK_SOL_STA_PRIM_FEAS => {
            println!("Feasible primal solution");
            for j in 0..numvar {
                println!("x[{}]: {:.4}", j, xx[j]);
            }
        },
        mosek::MSK_SOL_STA_UNKNOWN => {
            let prosta = task.get_pro_sta(mosek::MSK_SOL_ITG)?;
            match prosta {
                mosek::MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED => {
                    println!("Problem status Infeasible or unbounded");
                },
                mosek::MSK_PRO_STA_PRIM_INFEAS => {
                    println!("Problem status Infeasible.");
                },
                mosek::MSK_PRO_STA_UNKNOWN => {
                    println!("Problem status unknown.");
                },
                sta => {
                    println!("Other problem status: {:?}",sta);
                }
            }
        },
        sta => {
            println!("Other solution status: {:?}",sta);
        }
    }
    Ok(())
}
