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

fn main() {
    let mut M = Model::new_with_name("lo1");
    let x = M.variable("x", & greater_than(vec![0f64; 4]));

    let aptr  : Vec<usize> = vec![0, 3, 7, 9];

    let asubj : Vec<i64> = vec![x[0],x[1],x[2],
                                x[0],x[1],x[2],x[3],
                                    x[1],     x[3]];
    let acof  : Vec<f64> = vec![3.0,1.0,2.0,
                                2.0,1.0,3.0,1.0,
                                    2.0,    3.0];

    let c1 = M.constraint("c1",
                          expr(&aptr[0..2], asubj.as_slice(),acof.as_slice()),
                          & equal_to_scalar(30.0));

    let c2 = M.constraint("c2",
                          expr(&aptr[1..3], asubj.as_slice(),acof.as_slice()),
                          & greater_than_scalar(15.0));

    let c3 = M.constraint("c3",
                          expr(&aptr[2..4], asubj.as_slice(),acof.as_slice()),
                          & less_than_scalar(25.0));

    let c3 = M.constraint("x2",
                          expr(vec![0usize,1].as_slice(),
                               vec![x[1]].as_slice(),
                               vec![1.0].as_slice()),
                          & less_than_scalar(10.0));

    M.objective("obj",
                ObjectiveSense::Min,
                expr(vec![0usize, 4].as_slice(),
                     vec![x[0],x[1],x[2],x[3]].as_slice(),
                     vec![3.0, 1.0, 5.0, 1.0].as_slice()));

    M.solve();

    M.write_task("lo1.ptf");
}
