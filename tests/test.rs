extern crate conic_solver_api;
extern crate mosek;
use conic_solver_api::*;
use mosek::conic_solver_mosek::*;

#[test]
fn test_conic_model() {
    let mut m = MosekTask::new(None);

    let x = m.create_var_block( &Domain::Free{num : 3});
    let y = m.create_var_block( &Domain::Fixed{bound : &[1.0,2.0,3.0]});
    let z = m.create_var_block( &Domain::QuadCone{num : 1, dim : 3});
    let w = m.create_var_block( &Domain::PSDCone{num : 1, dim : 3});
    let c = m.create_con_block( &Domain::Free{num : 3});

    let mut xidxs = vec![0; m.get_var_block_size(x)];
    let mut widxs = vec![0; m.get_var_block_size(w)];
    let mut cidxs = vec![0; m.get_con_block_size(c)];
    m.get_var_block_indexes(x,xidxs.as_mut_slice());
    m.get_var_block_indexes(w,widxs.as_mut_slice());
    m.get_var_block_indexes(c,cidxs.as_mut_slice());

    m.write_task("conic.ptf");
}

#[test]
fn test_lo1() {
    let mut m = MosekTask::new(None);
    let x = m.create_var_block( &Domain::Lower{ bound  : &[0.0,0.0,0.0,0.0] });
    let c1 = m.create_con_block( &Domain::Fixed{ bound : &[30.0]});
    let c2 = m.create_con_block( &Domain::Lower{ bound : &[15.0]});
    let c3 = m.create_con_block( &Domain::Upper{ bound : &[25.0]});

    let mut xi = vec![0; 4];
    m.get_var_block_indexes(x,xi.as_mut_slice());

    let mut c1i = vec![0];
    let mut c2i = vec![0];
    let mut c3i = vec![0];
    m.get_con_block_indexes(c1,&mut c1i.as_mut_slice());
    m.get_con_block_indexes(c2,&mut c2i.as_mut_slice());
    m.get_con_block_indexes(c3,&mut c3i.as_mut_slice());

    //   x1  x2  x3  x4
    // | 3.0 1.0 2.0     |
    // | 2.0 1.0 3.0 1.0 |
    // |     2.0     3.0 |

    m.put_a_row_list(&[c1i[0],c2i[0],c3i[0]],
                     &[0,3,7,9],
                     &[xi[0],xi[1],xi[2],
                       xi[0],xi[1],xi[2],xi[3],
                             xi[1],      xi[3]],
                     &[3.0,1.0,2.0,
                       2.0,1.0,3.0,1.0,
                           2.0,    3.0]);

    m.put_objective_sense(ObjectiveSense::Maximize);
    m.put_objective(xi.as_slice(), &[3.0, 1.0, 5.0, 1.0]);

    let cx = m.create_con_block( &Domain::Upper{ bound : &[10.0]});
    let mut cxi = vec![0];
    m.get_con_block_indexes(cx,cxi.as_mut_slice());
    m.put_aij_list(cxi.as_slice(),&[xi[2]],&[1.0]);

    m.write_task("lo1_conic.ptf");
}


#[test]
fn test_lo1() {
    let mut m = MosekTask::new(None);

    let x    = m.create_var_block(Domain::Free{num : 3});
    let barx = m.create_var_block(Domain::PSDCone{num : 3, dim : 3});

    let c    = m.create_con_block(Domain::Fixed{bound : vec![1.0, 0.5]});

    let mut xi = vec![3];
    let mut ci = vec![2];
    m.get_var_block_indexes(x,xi.as_mut_slice());
    m.get_con_block_indexes(c,ci.as_mut_slice());

    m.put_a_row_list(c.as_slice(),
                     &[0,1,3],
                     &[xi[0],xi[1],ci[2],
                     &[1.0,  1.0,  1.0]]);


}
