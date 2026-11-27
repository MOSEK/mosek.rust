//!
//!   Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!   Purpose :   Demonstrates how to solve a  linear
//!               optimization problem using the MOSEK API
//!               and modify and re-optimize the problem.

extern crate mosek;
extern crate itertools;
use mosek::{Task,Boundkey,Objsense,Soltype};
use itertools::izip;

const INF : f64 = 0.0;

fn main() -> Result<(),String> {

    let numcon = 3;
    let numvar = 3;
    let c = &[1.5, 2.5, 3.0 ];
    let bkc = &[ Boundkey::UP,
                 Boundkey::UP,
                 Boundkey::UP ];
    let blc = &[ -INF,
                 -INF,
                 -INF ];
    let buc = &[ 100000.0,
                 50000.0,
                 60000.0 ];
    let bkx = &[ Boundkey::LO,
                 Boundkey::LO,
                 Boundkey::LO
                             ];
    let blx = &[ 0.0, 0.0, 0.0 ];
    let bux = &[ INF,
                 INF,
                 INF ];

    let asub = &[
        &[ 0i32, 1, 2 ],
        &[ 0i32, 1, 2 ],
        &[ 0i32, 1, 2 ] ];

    let aval = &[
        &[ 2.0, 3.0, 2.0 ],
        &[ 4.0, 2.0, 3.0 ],
        &[ 3.0, 3.0, 2.0 ] ];


    let mut task = Task::new().unwrap();
    /* Append the constraints. */
    task.append_cons(numcon)?;

    /* Append the variables. */
    task.append_vars(numvar)?;

    /* Put C. */
    for (j,&cj) in (0..numvar).zip(c.iter()) {
        task.put_c_j(j,cj)?;
    }
    /* Put constraint bounds. */
    for (i,&bki,&bli,&bui) in izip!(0..numcon,bkc,blc,buc) {
        task.put_con_bound(i, bki, bli, bui)?;
    }

    /* Put variable bounds. */
    for (j,&bki,&bli,&bui) in izip!(0..numvar,bkx,blx,bux) {
        task.put_var_bound(j, bki, bli, bui)?;
    }

    /* Put A. */
    if numcon > 0 {
        for (j,&asubj,&avalj) in izip!(0..numvar,asub,aval) {
            task.put_a_col(j,
                           asubj,
                           avalj)?;
        }
    }

    /* A maximization problem */
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    /* Solve the problem */
    let _trm = task.optimize()?;

    let mut xx = vec![0.0; task.get_num_var()? as usize];
    task.get_xx(Soltype::BAS, // Request the basic solution.
                xx.as_mut_slice())?;

    for (j,xj) in xx.iter().enumerate() {
        println!("x[{}]: {}",j,xj);
    }

    /****************** Make a change to the A matrix ******************/
    task.put_aij(0, 0, 3.0)?;
    let _trm = task.optimize()?;
    task.get_xx(Soltype::BAS, // Request the basic solution.
                xx.as_mut_slice())?;

    for (j,xj) in xx.iter().enumerate() {
        println!("x[{}]: {}",j,xj);
    }

    /***************** Add a new variable ******************************/
    /* Get index of new variable. */

    let varidx = task.get_num_var()?;

    /* Append a new variable x_3 to the problem */
    task.append_vars(1)?;
    let numvar = numvar + 1;

    /* Set bounds on new varaible */
    task.put_var_bound(varidx, Boundkey::LO, 0.0, INF)?;

    /* Change objective */
    task.put_c_j(varidx, 1.0)?;

    /* Put new values in the A matrix */
    let acolsub = &[0i32, 2];
    let acolval = &[4.0, 1.0];

    task.put_a_col(varidx, /* column index */
                   acolsub,
                   acolval)?;

    /* Change optimizer to simplex free and reoptimize */
    task.put_int_param(mosek::Iparam::OPTIMIZER, mosek::Optimizertype::FREE_SIMPLEX)?;
    let _trm = task.optimize()?;

    let mut xx = vec![0.0; task.get_num_var()? as usize];
    task.get_xx(Soltype::BAS, xx.as_mut_slice())?;

    for (j,xj) in (0..numvar).zip(xx.iter()) {
        println!("x[{}]: {}",j,xj);
    }

    /********************** Add a new constraint ***************************/
    /* Get index of new constraint. */
    let conidx = task.get_num_con()?;

    /* Append a new constraint */
    task.append_cons(1)?;
    let numcon = numcon + 1;

    /* Set bounds on new constraint */
    task.put_con_bound(conidx,
                       Boundkey::UP,
                       -INF,
                       30000.0)?;

    /* Put new values in the A matrix */
    let arowsub = &[0i32,   1,   2,   3  ];
    let arowval = &[1.0, 2.0, 1.0, 1.0 ];

    task.put_a_row(conidx, /* row index */
                   arowsub,
                   arowval)?;

    let _trm = task.optimize()?;

    task.get_xx(Soltype::BAS, // Request the basic solution.
                xx.as_mut_slice())?;

    for (j,xj) in (0..numvar).zip(xx.iter()) {
        println!("x[{}]: {}",j,xj);
    }

    /********************** Change constraint bounds ********************/
    let newbkc = &[Boundkey::UP,
                   Boundkey::UP,
                   Boundkey::UP,
                   Boundkey::UP];
    let newblc = &[-INF,
                   -INF,
                   -INF,
                   -INF];
    let newbuc = &[ 80000.0, 40000.0, 50000.0, 22000.0 ];

    task.put_con_bound_slice(0, numcon, newbkc, newblc, newbuc)?;

    let _ = task.optimize()?;

    task.get_xx(Soltype::BAS, // Request the basic solution.
                xx.as_mut_slice())?;

    for (j,xj) in (0..numvar).zip(xx.iter()) {
        println!("x[{}]: {}",j,xj);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
