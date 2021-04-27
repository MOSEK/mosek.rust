/*
   Copyright
       $$copyright
   File
       $${file}
   Description
       Basic portfolio example. For an initial fortune x0, maximize
       the expected return such that

       Maximize μ . x
       Such that
           budget: sum(x) = w + sum(x0)
           risk:   |  γ   |
                   |      | in C_q
                   |GT * x|
 */

extern crate mosek;

fn basic_markowitz(w  : f64,
                   mu  : &[f64],
                   x0 : &[f64],
                   gt : &[f64],
                   gammas : &[f64]) -> Result<Vec<f64>,String> {

    let env = match mosek::Env::new() {
        Some(e) => e,
        None => return Err("Failed to create env".to_string()) };
    /* Create the optimization task. */
    let mut task = match env.task() {
        Some(e) => e,
        None => return Err("Failed to create task".to_string()) };

    let n = mu.len();

    //task.put_stream_callback(mosek::MSK_STREAM_LOG, |msg| print!("{}",msg))?;
    //task.put_callback(|caller,_,_,_| { println!("caller = {}",caller); true })?;

    task.put_task_name("Portfolio")?;

    task.append_vars(n as i32)?;
    task.put_var_bound_slice(0,3,vec![mosek::MSK_BK_LO,mosek::MSK_BK_LO,mosek::MSK_BK_LO].as_slice(),vec![0.0,0.0,0.0].as_slice(),vec![0.0,0.0,0.0].as_slice())?;
    task.put_obj_sense(mosek::MSK_OBJECTIVE_SENSE_MAXIMIZE)?;
    task.put_c_list(vec![0,1,2].as_slice(),mu)?;

    // budget:
    task.append_cons(1)?;
    task.put_con_name(0,"budget")?;
    task.put_a_row(0,vec![0,1,2].as_slice(),vec![1.0,1.0,1.0].as_slice())?;
    let wealth : f64 = w + x0.iter().sum::<f64>();
    task.put_con_bound(0,mosek::MSK_BK_FX,wealth,wealth)?;

    // risk:
    task.append_afes(4)?;
    let afeidxs = vec![0,1,2,3];
    task.put_afe_f_row_list(afeidxs.as_slice(),vec![0,3,3,3].as_slice(), vec![0,0,3,6].as_slice(), vec![0,1,2, 0,1,2, 0,1,2].as_slice(), gt)?;
    let domidx = task.append_quadratic_cone_domain(4)?;
    task.append_acc(domidx,afeidxs.as_slice(),vec![0.0,0.0,0.0,0.0].as_slice())?;

    // Iterate over gammas
    let mut res = Vec::<f64>::new();

    for gamma in gammas.iter() {
        task.put_afe_g(0,*gamma)?;
        task.optimize()?;
        if mosek::MSK_SOL_STA_OPTIMAL != task.get_sol_sta(mosek::MSK_SOL_ITR)? {
            return Err("Result is not optimal".to_string());
        }


        let mut xx : Vec<f64> = Vec::new(); xx.resize(3,0.0);
        task.get_xx(mosek::MSK_SOL_ITR,xx.as_mut_slice())?;
        res.push(xx.iter().zip(mu.iter()).map(|(a,b)| (*a) * (*b)).sum());
    }

    Ok(res)
}


fn main() -> Result<(),String> {
    let w  = 1.0;
    let mu  = vec![0.1073, 0.0737, 0.0627];
    let x0 = vec![0.0, 0.0, 0.0];
    let gammas = vec![0.035, 0.040, 0.050, 0.060, 0.070, 0.080, 0.090];
    let gt = vec![ 0.166673333200005, 0.0232190712557243 ,  0.0012599496030238,
                   0.0              , 0.102863378954911  , -0.00222873156550421,
                   0.0              , 0.0                ,  0.0338148677744977 ];

    let res = basic_markowitz(w, mu.as_slice(), x0.as_slice(), gt.as_slice(), gammas.as_slice())?;

    for (g,x) in gammas.iter().zip(res.iter()) {
        println!("gamma={:.3} : {:.5}", *g, *x);
    }
    Ok(())
}
