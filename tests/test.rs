extern crate conic_solver_api;
extern crate mosek;

pub mod conic_model {
    use conic_solver_api::*;
    use mosek::conic_solver_mosek::*;



    #[test]
    fn test_conic_model() {
        let mut m = Solver::new();

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
        let mut m = Solver::new();
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
    fn test_sdo1() {
        let mut m = Solver::new();

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
}

pub mod affine_model {
    use mosek::affine_conic_api::*;

    #[test]
    fn test_index_manager() {
        let mut m = IndexManager::new(100);
        assert_eq!(m.capacity(),100);

        let i0 = m.allocate(5);
        let i1 = m.allocate(10);

        m.release(i0);
        let i2 = m.allocate(10);

        let mut r : Vec<usize> = vec![0; 10];
        m.get(i2,r.as_mut_slice());

        let expected_r : [usize;10] =  [0,1,2,3,4,15,16,17,18,19];
        assert!(r.iter().zip(expected_r.iter()).all(|(a,b)| a == b));

        m.validate().unwrap();
    }

    #[test]
    fn test_affine_model() {
        let mut m = Model::new();
        let x = m.variable(Some("x"),Domain::r().with_size(10));
        let y = m.variable(Some("y"),Domain::rotated_quadratic_cone().with_shape(&[3,3]).with_conedim(0));
        let z = m.variable(Some("z"),Domain::primal_exp_cone().with_rhs(&[1.0,2.0,3.0]));

        m.write_task("model.ptf");
    }
}