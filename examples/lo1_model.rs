extern crate mosek;
use mosek::model::*;
/*
min 3    1    5    1

    x1   x2   x3   x4
c1  3    1    2       = 30
c2  2    1    3    1  > 15
c3       2         3  < 25

x1 >  0
x2 in [0,10]
x3 >  0
x4 >  0

 */

fn main() -> Result<(),String> {
    let mut M = Model::with_name("lo1")?;
    let x = M.variable("x", & greater_than(vec![0f64; 4]))?;

    let asubi : Vec<usize> = vec![0,0,0,0,0,0,0,0,0];
    let asubj : Vec<usize> = vec![0,1,2,0,1,2,3,1,3];
    let acof  : Vec<f64>   = vec![3.0,1.0,2.0,2.0,1.0,3.0,1.0,2.0,3.0];

    let m1 = (1usize,4usize,&asubi[0..3],&asubj[0..3],&acof[0..3]);
    let m2 = (1usize,4usize,&asubi[3..7],&asubj[3..7],&acof[3..7]);
    let m3 = (1usize,4usize,&asubi[7..9],&asubj[7..9],&acof[7..9]);

    let c1 = M.constraint("c1",
                          &expr_mul_sparse(&m1,&x),
                          &equal_to_scalar(30.0))?;

    let c2 = M.constraint("c2",
                          &expr_mul_sparse(&m2,&x),
                          &greater_than_scalar(15.0))?;

    let c3 = M.constraint("c3",
                          &expr_mul_sparse(&m3,&x),
                          &less_than_scalar(25.0))?;

    let c3 = M.constraint("x2", &x.index(1), &less_than_scalar(10.0))?;

    M.objective("obj",
                ObjectiveSense::Min,
                &expr_dot(vec![3.0, 1.0, 5.0, 1.0].as_slice(), &x))?;

    M.solve()?;

    M.write_task("lo1.ptf")?;

    Ok(())
}
