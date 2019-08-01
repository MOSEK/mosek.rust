extern crate mosek;
use mosek::model::*;


/*
Purpose:
    Computes the optimal portfolio for a given risk

Input:
    n: Number of assets
    mu: An n dimmensional vector of expected returns
    GT: A matrix with n columns so (GT")*GT  = covariance matrix"
    x0: Initial holdings
    w: Initial cash holding
    gamma: Maximum risk (=std. dev) accepted

Output:
    Optimal expected return and the optimal portfolio
*/
fn basic_markowitz(n : usize,
                   mu : &[f64],
                   GT : &(usize,usize,Vec<f64>),
                   x0 : &[f64],
                   w  : f64,
                   gamma  : f64) -> Result<f64,String> {
    let mut M = Model::with_name("Basic Markowitz")?;

    // Redirect log output from the solver to stdout for debugging.
    //M.set_log_handler(|msg| print!("{}",msg))?;
    M.clear_log_handler()?;

    // Defines the variables (holdings). Shortselling is not allowed.
    let x = M.variable("x", &greater_than(vec![0.0; n]))?;

    //  Maximize expected return
    M.objective("obj", ObjectiveSense::Max, &expr_dot(&mu,&x))?;

    // The amount invested  must be identical to intial wealth
    let sumx0 : f64 = x0.iter().sum();
    M.constraint("budget", &expr_sum(&x), &equal_to_scalar(w + sumx0))?;

    // Imposes a bound on the risk
    M.constraint("risk",&expr_stack_2(&gamma, &expr_mul(&GT,&x)), &in_second_order_cone(1+n))?;

    // Solves the model.
    M.solve()?;

    M.write_task(format!("portfolio-{:.2}.ptf",gamma).as_str());

    let mut xlvl = vec![0.0; n];
    M.level(&x,xlvl.as_mut_slice())?;

    Ok(mu.iter().zip(xlvl.iter()).map(|(a,b)| a*b).sum())
}


fn main() -> Result<(),String> {
    let n : usize = 3;
    let w = 1.0;
    let mu = vec![0.1073, 0.0737, 0.0627];
    let x0 = vec![0.0,    0.0,    0.0];
    let gammas = vec![0.035, 0.040, 0.050, 0.060, 0.070, 0.080, 0.090];
    let GT = (3,3,vec![0.166673333200005, 0.0232190712557243 ,  0.0012599496030238,
                       0.0              , 0.102863378954911  , -0.00222873156550421,
                       0.0              , 0.0                ,  0.0338148677744977 ]);

    for gamma in gammas {
        let res = basic_markowitz(n,mu.as_slice(),&GT,x0.as_slice(),w,gamma)?;
        println!("Expected return: {0:.4}  St deviation: {1:.4} ", res,gamma);
    }
    Ok(())
}
