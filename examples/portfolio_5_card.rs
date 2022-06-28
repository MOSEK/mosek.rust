/*
  File : $${file}

  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.

  Description :  Implements a basic portfolio optimization model
                 with cardinality constraints on number of assets traded.

  Maximize mu'x
  Such That ACC: [ gamma, G'x] in Q^{n+1}
            sum(x) = w0+sum(x0)
            sum(y) < p
            DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
            y free
            x > 0
  Where f_i is the fixed cost of a transaction in asset i,
        g_i is the cost per unit of a transaction in asset i
  
 */
extern crate mosek;
use mosek::{Task,Objsense,Streamtype,Soltype};

#[allow(non_snake_case)]
fn portfolio(n : i32,
             mu : &[f64],
             GT : &[f64],
             x0  : &[f64],
             gamma : f64,
             p  : i32,
             w : f64) -> Result<Vec<f64>,String> {

    /* Create the optimization task. */
    let mut task = match Task::new() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };

    let k = (GT.len() / n as usize) as i32;
    task.put_stream_callback(Streamtype::LOG, |msg| print!("{}",msg))?;

    /* Compute total wealth */
    let w0 = w + x0.iter().sum::<f64>();

    task.append_cons(2i32)?;
    task.append_vars(3*n)?;

    let x : Vec<i32> = (0..n).collect();
    let y : Vec<i32> = (n..2*n).collect();

    for (i,j) in (0..n).zip(x.iter()) {
        task.put_var_name(*j,    format!("x[{}]",i).as_str())?;
    }
    for (i,j) in (0..n).zip(y.iter()) {
        task.put_var_name(*j,  format!("y[{}]",i).as_str())?;
    }

    // objective
    task.put_obj_sense(Objsense::MAXIMIZE)?;
    for (j,mui) in x.iter().zip(mu.iter()) {
        task.put_c_j(*j, *mui)?;
    }


    // budget constraint
    task.put_con_name(0,"budget")?;
    task.put_a_row(0,
                   x.as_slice(),
                   (0..n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
    task.put_con_bound(0i32,mosek::Boundkey::FX,w0,w0)?;
    // cardinality constraint
    task.put_con_name(1,"cardinality")?;
    task.put_a_row(1,
                   y.as_slice(),
                   (0..n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
    task.put_con_bound(1,mosek::Boundkey::UP,p as f64,p as f64)?;

    let mut afei = 0;
    // (gamma,G'x) in Q
    {
        task.append_afes(k as i64+1)?;
        let acci = task.get_num_acc()?;
        let dom = task.append_quadratic_cone_domain(k as i64+1)?;
        task.append_acc(dom,
                        (afei..afei+k as i64+1).collect::<Vec<i64>>().as_slice(),
                        (0..k+1).map(|_| 0.0).collect::<Vec<f64>>().as_slice())?;
        task.put_acc_name(acci,"GT")?;
        task.put_afe_g(0,gamma)?;
        let mut l = 0;
        for i in 0..k {
            for j in 0..n {
                if GT[l] != 0.0 {
                    task.put_afe_f_entry(i as i64+1,j as i32,GT[l])?;
                }
                l += 1;
            }
        }
        afei += k as i64 +1;
    }

    //DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
    {
        task.append_djcs(n as i64)?;
        let domeq = task.append_rzero_domain(1)?;
        for (((xi,yi),x0i),i) in x.iter().zip(y.iter()).zip(x0.iter()).zip(0..n) {
            task.append_afes(3)?;
            // y_j = 0
            task.put_afe_f_entry(afei,*yi,1.0)?;
            // x_j = x0_j
            task.put_afe_f_entry(afei+1,*xi,1.0)?;
            // y_j = 1
            task.put_afe_f_entry(afei+2,*yi,1.0)?;
            task.put_djc(i as i64,
                         &[domeq,domeq,domeq],
                         &[afei,afei+1,afei+2],
                         &[0.0, *x0i, 1.0],
                         &[2,1])?;
            afei += 3;
        }
    }

    let _ = task.optimize()?;

    /* Dump the problem to a human readable OPF file. */
    task.write_data("portfolio_5_card.ptf")?;

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(Streamtype::MSG)?;

    let mut xx = vec![0.0;(2*n) as usize];
    task.get_xx(Soltype::ITG, xx.as_mut_slice())?;
    // let expret = xx[0..n as usize].iter().zip(mu.iter()).map(|(a,b)| a*b).sum::<f64>();
    Ok(xx[0..n as usize].to_vec())
}


#[allow(non_snake_case)]
fn main() -> Result<(),String> {
    let n       = 3i32;
    let w       = 1.0;
    let x0      = vec![0.0, 0.0, 0.0];
    let gamma   = 0.05;
    let mu      = vec![0.1073,  0.0737,  0.0627];
    let GT      = vec![0.1667,  0.0232,  0.0013,
                       0.0000,  0.1033, -0.0022,
                       0.0000,  0.0000,  0.0338 ];
    for k in 1..3 {
        let x = portfolio(n,
                          mu.as_slice(),
                          GT.as_slice(),
                          x0.as_slice(),
                          gamma,
                          k,
                          w)?;
        let expret = x.iter().zip(mu.iter()).map(|(xi,mui)| xi * mui).sum::<f64>();
        println!("Bound {}: x = {:?}", k,x);
        println!("  Return: {:.5e}\n", expret);
    }
    Ok(())
}

