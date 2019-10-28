extern crate conic_solver_api;
use self::conic_solver_api::*;
use conic_solver_mosek::*;

#[test]
fn test_conic_model() {
    let m = MosekTask::new(None);

    let x = m.create_var_block( &Domain::Free{num : 3});
    let y = m.create_var_block( &Domain::Fixed{bound : &[1.0,2.0,3.0]});
    let z = m.create_var_block( &Domain::QuadCone{num : 1, dim : 3});
    let w = m.create_var_block( &Domain::PSDCone{num : 1, dim : 3});

    let c = m.create_con_block( &Domain::Free{num : 3});

    let xidxs = vec![0.0; m.get_var_block_size(x)];
    let widxs = vec![0.0; m.get_var_block_size(w)];
    let cidxs = vec![0.0; m.get_con_block_size(c)];
    m.get_var_block_idxs(x,xidxs);
    m.get_var_block_idxs(w,widxs);
    m.get_var_block_idxs(c,cidxs);
    
    
    
}

