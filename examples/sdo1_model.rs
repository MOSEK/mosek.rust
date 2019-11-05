extern crate conic_solver_api;
extern crate mosek;
use conic_solver_api::*;
use mosek::conic_solver_mosek::*;

const INF : f64 = 0.0;

//           |2 1 .|
// Minimize  |1 2 1|*X + x[0]
//           |. 1 2|
//
// Such that |1 . .|
//           |. 1 .|*X + x[0] = 1.0
//           |. . 1|
//
//           |1 1 1|
//           |1 1 1|*X + x[1] + x[2] = 0.5
//           |1 1 1|

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
    m.solve();

    println!("Number of solutions : {}",m.get_num_solutions());

    for i in 0..m.get_num_solutions() {
      println!("Solution status: {:?}",m.get_solution_status(i).unwrap());
      println!("Problem status:  {:?}",m.get_problem_status(i).unwrap());
      println!("  ## {:?} ##",m.get_solution_type(i));

      let mut level_x = vec![0.0; 3];
      let mut level_X = vec![0.0; 6];
      let mut dual_x  = vec![0.0; 3];
      let mut dual_X  = vec![0.0; 6];
      m.get_primal_var_solution(i,xi.as_slice(),level_x.as_mut_slice());
      m.get_dual_var_solution(  i,xi.as_slice(),dual_x.as_mut_slice());      
      m.get_primal_var_solution(i,barxi.as_slice(),level_X.as_mut_slice());
      m.get_dual_var_solution(  i,barxi.as_slice(),dual_X.as_mut_slice());      

      println!("  xx = {:?}",level_x);
      println!("  ss = {:?}",dual_x);
      println!("  barx = {:?}",level_X);
      println!("  bars = {:?}",dual_X);
    }
    m.write_task("sdo1.ptf");
    m.write_task("sdo1.opf");
}
