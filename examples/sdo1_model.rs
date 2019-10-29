extern crate conic_solver_api;
extern crate mosek;
use conic_solver_api::*;
use mosek::conic_solver_mosek::*;

const INF : f64 = 0.0;

fn main() {
    let mut m = MosekTask::new(None);

    let x    = m.create_var_block(&Domain::QuadCone{num : 1, dim : 3});
    let barx = m.create_var_block(&Domain::PSDCone{num : 1, dim : 3});
    let c    = m.create_con_block(&Domain::Fixed{bound : &[1.0, 0.5]});

    let mut xi    = vec![0;3];
    let mut barxi = vec![0;6];
    let mut ci    = vec![0;2];
    m.get_var_block_indexes(x,xi.as_mut_slice());
    m.get_var_block_indexes(barx,barxi.as_mut_slice());
    m.get_con_block_indexes(c,ci.as_mut_slice());

    println!("{:?}, {:?}",xi,barxi);
    m.put_a_row_list(ci.as_slice(),
                     &[0,4,12],
                     &[xi[0],barxi[0],barxi[2],barxi[5],
                       xi[1],xi[2],   barxi[0],barxi[1],barxi[2], barxi[3],barxi[4],barxi[5]],
                     &[1.0,  1.0,     1.0,     1.0,
                       1.0,  1.0,     1.0,     2.0,     1.0,      2.0,     2.0,     1.0]);

    m.put_objective(&[xi[0],barxi[0],barxi[1],barxi[2],barxi[4],barxi[5]],
                    &[1.0,  2.0,     2.0,     2.0,     2.0,     2.0]);
    m.put_objective_sense(ObjectiveSense::Minimize);
    m.write_task("sdo1.ptf");
}
