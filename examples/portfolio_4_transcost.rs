
/*
  File : $${file}

  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.

  Description :  Implements a basic portfolio optimization model
                 with fixed setup costs and transaction costs
                 as a mixed-integer problem.

  Maximize mu'x
  Such That ACC: [ gamma, G'x] in Q^{n+1}
            sum(x) + f'y + g'z = w0+sum(x0)
            ACC: z_j > |x0_j-x_j|
            DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
                    // z_j < U_j y_j, y in {0,1}
            y free
            x > 0
  Where f_i is the fixed cost of a transaction in asset i,
        g_i is the cost per unit of a transaction in asset i
 */
extern crate mosek;

fn portfolio(n : i32,
             mu : &[f64],
             f  : &[f64],
             g  : &[f64],
             GT : &[f64],
             x0  : &[f64],
             gamma : f64,
             w : f64) -> Result<(),String> {
    let env = match mosek::Env::new() {
        Some(e) => e,
        None => return Err("Failed to create env".to_string()),
    };
    /* Create the optimization task. */
    let mut task = match env.task() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()),
    };

    let k = (GT.len() / n as usize) as i32;
    task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;


    /* Compute total wealth */
    let w0 = w + x0.iter().sum::<f64>();

    task.append_cons(1i32)?;
    task.append_vars(3*n)?;

    for i in 0i32..n {
        task.put_var_name(i,    format!("x[{}]",i).as_str())?;
        task.put_var_name(i+n,  format!("y[{}]",i).as_str())?;
        task.put_var_name(i+2*n,format!("z[{}]",i).as_str())?;
    }

    task.put_var_bound_slice(0i32,3*n,
                             vec![mosek::MSK_BK_FR; 3*n as usize].as_slice(),
                             vec![0.0; 3*n as usize].as_slice(),
                             vec![0.0; 3*n as usize].as_slice())?;

    let x_base = 0i32;
    let y_base = n;
    let z_base = 2*n;

    /* Constraints. */
    task.put_con_name(0,"budget")?;
    task.put_a_row(0i32,
                   (0i32..3*n).collect::<Vec<i32>>().as_slice(),
                   (0..n).map(|_| 1.0).chain(f.iter().map(|v| *v)).chain(g.iter().map(|v| *v)).collect::<Vec<f64>>().as_slice())?;
    task.put_con_bound(0i32,mosek::MSK_BK_FX,w0,w0)?;

    // objective
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE)?;
    for (i,mui) in (0..n).zip(mu.iter()) {
        task.put_c_j(x_base+i, *mui)?;
    }


    
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

    // z_i > |x_i-x0_i|
    {
        let dom = task.append_rplus_domain(n as i64)?;
        // z_i > x_i - x0_i <=> z_i - x_i + x0_i > 0
        task.append_afes(n as i64)?;
        task.put_afe_f_entry_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                                  (x_base..x_base+n).chain(z_base..z_base+n).collect::<Vec<i32>>().as_slice(),
                                  (0..n).map(|_| -1.0 ).chain((0..n).map(|_| 1.0)).collect::<Vec<f64>>().as_slice())?;
        task.put_afe_g_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                            x0)?;
        task.append_acc(dom,
                        (afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                        (0..n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
        afei += n as i64;
        // z_i > x0_i - x_i <=> z_i + z_i - x0_i > 0
        task.append_afes(n as i64)?;
        task.put_afe_f_entry_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                                  (x_base..x_base+n).chain(z_base..z_base+n).collect::<Vec<i32>>().as_slice(),
                                  (0..2*n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
        task.put_afe_g_list((afei..afei+n as i64).chain(afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                            x0.iter().map(|v| -v).collect::<Vec<f64>>().as_slice())?;
        task.append_acc(dom,
                        (afei..afei+n as i64).collect::<Vec<i64>>().as_slice(),
                        (0..n).map(|_| 1.0).collect::<Vec<f64>>().as_slice())?;
        afei += n as i64;
    }

    //DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
    {
        task.append_djcs(n as i64)?;
        let domeq = task.append_rzero_domain(1)?;
        for i in 0..n {
            task.append_afes(3)?;
            // y_j = 0
            task.put_afe_f_entry(afei,y_base+i,1.0)?;
            // x_j = x0_j
            task.put_afe_f_entry(afei+1,x_base+i,1.0)?;
            // y_j = 1
            task.put_afe_f_entry(afei+2,y_base+i,1.0)?;
            task.put_djc(i as i64,
                         &[domeq,domeq,domeq],
                         &[afei,afei+1,afei+2],
                         &[0.0, x0[i as usize], 1.0],
                         &[2,1])?;
            afei += 3;
        }
    }


    let _ = task.optimize()?;

    /* Dump the problem to a human readable OPF file. */
    task.write_data("portfolio_4_transcost.ptf")?;

    /* Display the solution summary for quick inspection of results. */
    task.solution_summary(mosek::MSK_STREAM_MSG)?;

    let mut xx = vec![0.0;(3*n) as usize];
    task.get_xx(mosek::MSK_SOL_ITG, xx.as_mut_slice())?;
    let expret = xx[0..n as usize].iter().zip(mu.iter()).map(|(a,b)| a*b).sum::<f64>();


    //for j in 0..n {
      //MOSEKCALL(res, MSK_getxxslice(task, MSK_SOL_ITG, offsetx + j, offsetx + j + 1, &xj));
      //expret += mu[j] * xx[j];
    //}

    //MOSEKCALL(res, MSK_getxxslice(task, MSK_SOL_ITG, offsets + 0, offsets + 1, &stddev));

    println!("\nExpected return {:.4e} for gamma {:.4e}\n", expret, gamma);
    Ok(())
}

fn main() -> Result<(),String> {
    let n       = 3i32;
    let w       = 1.0;
    let x0      = vec![0.0, 0.0, 0.0];
    let gamma   = 0.05;
    let mu      = vec![0.1073,  0.0737,  0.0627];
    let f       = vec![0.01, 0.01, 0.01];
    let g       = vec![0.001, 0.001, 0.001];
    let GT      = vec![0.1667,  0.0232,  0.0013,
                       0.0000,  0.1033, -0.0022,
                       0.0000,  0.0000,  0.0338 ];
    portfolio(n,
              mu.as_slice(),
              f.as_slice(),
              g.as_slice(),
              GT.as_slice(),
              x0.as_slice(),
              gamma,
              w)?;

    Ok(())
}
