
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
    let x_base = 0i32;
    let y_base = n;
    let z_base = 2*n;

    /* Constraints. */
    task.put_con_name(0,"budget");
    task.put_a_row(0i32,
                   (0i32..3*n).collect().as_slice(),
                   (0..n).map(|_| 1.0).chain(f.iter()).chain(g.iter()).collect().as_slice())?;
    task.put_con_bound(0i32,mosek::MSK_BK_FX,w0,w0);

    let mut afei = 0;
    // (gamma,G'x) in Q
    {
        task.append_afes(k+1)?;
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
        afei += k+1;
    }

    {
        let dom = task.append_r_plus_domain(n as i32)?;
        // z_i > x_i - x0_i <=> z_i - x_i + x0_i > 0
        task.append_afes(n as i64)?;
        task.put_afe_f_entry_list((afei..afei+n).chain(afei..afei+n).collect().as_slice(),
                                  (x_base..x_base+n).chain(z_base..z_base+n).collect().as_slice(),
                                  (0..n).map(|_| -1.0).chain((0..n).map(|_| 1.0).collect().as_slice()))?;
        task.put_afe_g_list((afei..afei+n).chain(afei..afei+n).collect().as_slice(),
                            x0.as_slice())?;
        task.append_acc(dom,
                        (afei..afei+n).collect().as_slice(),
                        (0..n).map(|_| 1.0).collect().as_slice());
        afei += n;
        // z_i > x0_i - x_i <=> z_i + z_i - x0_i > 0
        task.append_afes(n as i64)?;
        task.put_afe_f_entry_list((afei..afei+n).chain(afei..afei+n).collect().as_slice(),
                                  (x_base..x_base+n).chain(z_base..z_base+n).collect().as_slice(),
                                  (0..2*n).map(|_| 1.0).collect().as_slice())?;
        task.put_afe_g_list((afei..afei+n).chain(afei..afei+n).collect().as_slice(),
                            x0.iter().map(|v| -v).collect().as_slice())?;
        task.append_acc(dom,
                        (afei..afei+n).collect().as_slice(),
                        (0..n).map(|_| 1.0).collect().as_slice());
        afei += n;
    }

    {
        //DJC:  [ y_j == 0 AND x0_j == x_j ] OR y_j == 1
        task.append_djcs(n as i64);
        let domeq = task.append_r_zero_domain(1)?
        for i in 0..n {
            task.append_afes(3);
            task.put_afe_f_entry(afei,y_base+i,1.0)?;
            task.put_afe_f_entry(afei+1,x_base+i,1.0)?;
            task.put_afe_f_entry(afei+2,y_base+i,1.0)?
            task.put_djc(i as i64,
                         &[domeq,domeq,domeq],
                         &[afei,afei+1,afei+2],
                         &[0.0, x0[i], 1.0],
                         &[2,1])
            );
        }
    }

    for i in 0..n {
        task.putconbound(1 + i, mosek::MSK_BK_FX, 0.0, 0.0)?;
        task.putconname(1 + i, format!("GT[{}]",1+i).as_str())?;
    }

    for i in 0..n {
        task.putconbound(1 + n + i, mosek::MSK_BK_LO, -x0[i], 0)?;
        task.putconname(1 + n + i, format!("zabs1[{}]",1+i).as_str())?;
    }

    for i in 0..n {
        task.putconbound(1 + 2 * n + i, mosek::MSK_BK_LO, x0[i], 0)?;
        task.putconname(1 + 2 * n + i, format!("zabs2[{}]",1+i).as_str())?;
    }

    for i in 0..n {
        task.putconbound(1 + 3 * n + i, mosek::MSK_BK_UP, 0, 0.0);
        task.putconname(task, 1 + 3 * n + i, format!("indicator[{}]",1+i).as_str());
    }

    /* Offsets of variables into the (serialized) API variable. */
    let offsetx = 0i32;
    let offsets = n;
    let offsett = n + 1;
    let offsetz = 2 * n + 1;
    let offsety = 3 * n + 1;

    /* Variables. */
    task.appendvars(4 * n + 1)?;

    /* x variables. */
    for j in 0..n {
        task.put_c_j(offsetx+j,mu[j])?;
        task.put_a_ij(0i32,);
    }


    for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putcj(task, offsetx + j, mu[j]));
    MOSEKCALL(res, MSK_putaij(task, 0, offsetx + j, 1.0));
    for (k = 0; k < n; ++k)
      if (GT[k][j] != 0.0)
        MOSEKCALL(res, MSK_putaij(task, 1 + k, offsetx + j, GT[k][j]));
    MOSEKCALL(res, MSK_putaij(task, 1 + n + j, offsetx + j, -1.0));
    MOSEKCALL(res, MSK_putaij(task, 1 + 2 * n + j, offsetx + j, 1.0));

    MOSEKCALL(res, MSK_putvarbound(task, offsetx + j, MSK_BK_LO, 0.0, MSK_INFINITY));
    sprintf(buf, "x[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsetx + j, buf));
  }

  /* s variable. */
  MOSEKCALL(res, MSK_putvarbound(task, offsets + 0, MSK_BK_FX, gamma, gamma));
  sprintf(buf, "s");
  MOSEKCALL(res, MSK_putvarname(task, offsets + 0, buf));

  /* t variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putaij(task, 1 + j, offsett + j, -1.0));
    MOSEKCALL(res, MSK_putvarbound(task, offsett + j, MSK_BK_FR, -MSK_INFINITY, MSK_INFINITY));
    sprintf(buf, "t[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsett + j, buf));
  }

  /* z variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putaij(task, 0, offsetz + j, g[j]));    
    MOSEKCALL(res, MSK_putaij(task, 1 + 1 * n + j, offsetz + j, 1.0));
    MOSEKCALL(res, MSK_putaij(task, 1 + 2 * n + j, offsetz + j, 1.0));
    MOSEKCALL(res, MSK_putaij(task, 1 + 3 * n + j, offsetz + j, 1.0));    
    MOSEKCALL(res, MSK_putvarbound(task, offsetz + j, MSK_BK_FR, -MSK_INFINITY, MSK_INFINITY));
    sprintf(buf, "z[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsetz + j, buf));
  }

  /* y variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putaij(task, 0, offsety + j, f[j]));    
    MOSEKCALL(res, MSK_putaij(task, 1 + 3 * n + j, offsety + j, -U));    
    MOSEKCALL(res, MSK_putvarbound(task, offsety + j, MSK_BK_RA, 0.0, 1.0));
    MOSEKCALL(res, MSK_putvartype(task, offsety + j, MSK_VAR_TYPE_INT));
    sprintf(buf, "y[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsety + j, buf));
  }

  if (res == MSK_RES_OK)
  {
    /* sub should be n+1 long i.e. the dimmension of the cone. */
    MSKint32t *sub = (MSKint32t *) MSK_calloctask(task, 3 >= n + 1 ? 3 : n + 1, sizeof(MSKint32t));

    if (sub)
    {
      /* Add quadratic cone */
      sub[0] = offsets + 0;
      for (j = 0; j < n; ++j)
        sub[j + 1] = offsett + j;

      MOSEKCALL(res, MSK_appendcone(task, MSK_CT_QUAD, 0.0, n + 1, sub));
      MOSEKCALL(res, MSK_putconename(task, 0, "stddev"));

      MSK_freetask(task, sub);
    }
    else
      res = MSK_RES_ERR_SPACE;
  }

  MOSEKCALL(res, MSK_putobjsense(task, MSK_OBJECTIVE_SENSE_MAXIMIZE));

#if 0
  /* no log output. */
  MOSEKCALL(res, MSK_putintparam(task, MSK_IPAR_LOG, 0));
#endif

#if 0
  /* Dump the problem to a human readable OPF file. */
  MOSEKCALL(res, MSK_writedata(task, "dump.opf"));
#endif

  MOSEKCALL(res, MSK_optimizetrm(task, &trmcode));

#if 1
  /* Display the solution summary for quick inspection of results. */
  MSK_solutionsummary(task, MSK_STREAM_MSG);
#endif
  
  if (res == MSK_RES_OK)
  {
    expret = 0.0;
    stddev = 0.0;

    for (j = 0; j < n; ++j)
    {
      MOSEKCALL(res, MSK_getxxslice(task, MSK_SOL_ITG, offsetx + j, offsetx + j + 1, &xj));
      expret += mu[j] * xj;
    }

    MOSEKCALL(res, MSK_getxxslice(task, MSK_SOL_ITG, offsets + 0, offsets + 1, &stddev));

    printf("\nExpected return %e for gamma %e\n", expret, stddev);
  }
}

fn main() -> Result<(),String> {
    let n       = 3i32;
    let w       = 1.0;
    let x0      = vec![0.0, 0.0, 0.0];
    let gamma   = 0.05;
    let mu[]    = vec![0.1073,  0.0737,  0.0627];
    let f[]     = vec![0.01, 0.01, 0.01],
    let g[]     = vec![0.001, 0.001, 0.001];
    let GT[][3] = vec![0.1667,  0.0232,  0.0013,
                       0.0000,  0.1033, -0.0022,
                       0.0000,  0.0000,  0.0338 ];
    portfolio(n,
              mu.as_slice(),
              f.as_slice(),
              g.as_slice(),
              m.as_slice(),
              GT.as_slice(),
              x0.as_slice(),
              gamma.as_slice(),
              w)?;

    Ok(());

    
  /* Initial setup. */
  env  = NULL;
  task = NULL;
  MOSEKCALL(res, MSK_makeenv(&env, NULL));
  MOSEKCALL(res, MSK_maketask(env, 0, 0, &task));
  MOSEKCALL(res, MSK_linkfunctotaskstream(task, MSK_STREAM_LOG, NULL, printstr));

  /* Compute total wealth */
  U = w;
  for (k = 0; k < n; ++k) U += x0[k];

  /* Constraints. */
  MOSEKCALL(res, MSK_appendcons(task, 1 + 4 * n));
  MOSEKCALL(res, MSK_putconbound(task, 0, MSK_BK_FX, w, w));
  sprintf(buf, "%s", "budget");
  MOSEKCALL(res, MSK_putconname(task, 0, buf));

  for (i = 0; i < n; ++i)
  {
    MOSEKCALL(res, MSK_putconbound(task, 1 + i, MSK_BK_FX, 0.0, 0.0));
    sprintf(buf, "GT[%d]", 1 + i);
    MOSEKCALL(res, MSK_putconname(task, 1 + i, buf));
  }

  for (i = 0; i < n; ++i)
  {
    MOSEKCALL(res, MSK_putconbound(task, 1 + n + i, MSK_BK_LO, -x0[i], MSK_INFINITY));
    sprintf(buf, "zabs1[%d]", 1 + i);
    MOSEKCALL(res, MSK_putconname(task, 1 + n + i, buf));
  }

  for (i = 0; i < n; ++i)
  {
    MOSEKCALL(res, MSK_putconbound(task, 1 + 2 * n + i, MSK_BK_LO, x0[i], MSK_INFINITY));
    sprintf(buf, "zabs2[%d]", 1 + i);
    MOSEKCALL(res, MSK_putconname(task, 1 + 2 * n + i, buf));
  }

  for (i = 0; i < n; ++i)
  {
    MOSEKCALL(res, MSK_putconbound(task, 1 + 3 * n + i, MSK_BK_UP, -MSK_INFINITY, 0.0));
    sprintf(buf, "indicator[%d]", 1 + i);
    MOSEKCALL(res, MSK_putconname(task, 1 + 3 * n + i, buf));
  }

  /* Offsets of variables into the (serialized) API variable. */
  offsetx = 0;
  offsets = n;
  offsett = n + 1;
  offsetz = 2 * n + 1;
  offsety = 3 * n + 1;

  /* Variables. */
  MOSEKCALL(res, MSK_appendvars(task, 4 * n + 1));

  /* x variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putcj(task, offsetx + j, mu[j]));
    MOSEKCALL(res, MSK_putaij(task, 0, offsetx + j, 1.0));
    for (k = 0; k < n; ++k)
      if (GT[k][j] != 0.0)
        MOSEKCALL(res, MSK_putaij(task, 1 + k, offsetx + j, GT[k][j]));
    MOSEKCALL(res, MSK_putaij(task, 1 + n + j, offsetx + j, -1.0));
    MOSEKCALL(res, MSK_putaij(task, 1 + 2 * n + j, offsetx + j, 1.0));

    MOSEKCALL(res, MSK_putvarbound(task, offsetx + j, MSK_BK_LO, 0.0, MSK_INFINITY));
    sprintf(buf, "x[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsetx + j, buf));
  }

  /* s variable. */
  MOSEKCALL(res, MSK_putvarbound(task, offsets + 0, MSK_BK_FX, gamma, gamma));
  sprintf(buf, "s");
  MOSEKCALL(res, MSK_putvarname(task, offsets + 0, buf));

  /* t variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putaij(task, 1 + j, offsett + j, -1.0));
    MOSEKCALL(res, MSK_putvarbound(task, offsett + j, MSK_BK_FR, -MSK_INFINITY, MSK_INFINITY));
    sprintf(buf, "t[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsett + j, buf));
  }

  /* z variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putaij(task, 0, offsetz + j, g[j]));    
    MOSEKCALL(res, MSK_putaij(task, 1 + 1 * n + j, offsetz + j, 1.0));
    MOSEKCALL(res, MSK_putaij(task, 1 + 2 * n + j, offsetz + j, 1.0));
    MOSEKCALL(res, MSK_putaij(task, 1 + 3 * n + j, offsetz + j, 1.0));    
    MOSEKCALL(res, MSK_putvarbound(task, offsetz + j, MSK_BK_FR, -MSK_INFINITY, MSK_INFINITY));
    sprintf(buf, "z[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsetz + j, buf));
  }

  /* y variables. */
  for (j = 0; j < n; ++j)
  {
    MOSEKCALL(res, MSK_putaij(task, 0, offsety + j, f[j]));    
    MOSEKCALL(res, MSK_putaij(task, 1 + 3 * n + j, offsety + j, -U));    
    MOSEKCALL(res, MSK_putvarbound(task, offsety + j, MSK_BK_RA, 0.0, 1.0));
    MOSEKCALL(res, MSK_putvartype(task, offsety + j, MSK_VAR_TYPE_INT));
    sprintf(buf, "y[%d]", 1 + j);
    MOSEKCALL(res, MSK_putvarname(task, offsety + j, buf));
  }

  if (res == MSK_RES_OK)
  {
    /* sub should be n+1 long i.e. the dimmension of the cone. */
    MSKint32t *sub = (MSKint32t *) MSK_calloctask(task, 3 >= n + 1 ? 3 : n + 1, sizeof(MSKint32t));

    if (sub)
    {
      /* Add quadratic cone */
      sub[0] = offsets + 0;
      for (j = 0; j < n; ++j)
        sub[j + 1] = offsett + j;

      MOSEKCALL(res, MSK_appendcone(task, MSK_CT_QUAD, 0.0, n + 1, sub));
      MOSEKCALL(res, MSK_putconename(task, 0, "stddev"));

      MSK_freetask(task, sub);
    }
    else
      res = MSK_RES_ERR_SPACE;
  }

  MOSEKCALL(res, MSK_putobjsense(task, MSK_OBJECTIVE_SENSE_MAXIMIZE));

#if 0
  /* no log output. */
  MOSEKCALL(res, MSK_putintparam(task, MSK_IPAR_LOG, 0));
#endif

#if 0
  /* Dump the problem to a human readable OPF file. */
  MOSEKCALL(res, MSK_writedata(task, "dump.opf"));
#endif

  MOSEKCALL(res, MSK_optimizetrm(task, &trmcode));

#if 1
  /* Display the solution summary for quick inspection of results. */
  MSK_solutionsummary(task, MSK_STREAM_MSG);
#endif
  
  if (res == MSK_RES_OK)
  {
    expret = 0.0;
    stddev = 0.0;

    for (j = 0; j < n; ++j)
    {
      MOSEKCALL(res, MSK_getxxslice(task, MSK_SOL_ITG, offsetx + j, offsetx + j + 1, &xj));
      expret += mu[j] * xj;
    }

    MOSEKCALL(res, MSK_getxxslice(task, MSK_SOL_ITG, offsets + 0, offsets + 1, &stddev));

    printf("\nExpected return %e for gamma %e\n", expret, stddev);
  }

  MSK_deletetask(&task);
  MSK_deleteenv(&env);

  return (0);
}
