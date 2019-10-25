extern crate conic_solver_api;
use self::conic_solver_api::*;

pub struct Solution {
    tp      : SolType,
    prosta  : ProblemStatus,
    psolsta : SolutionStatus,
    dsolsta : SolutionStatus,
    skx     : Vec<StaKey>,
    xx      : Vec<f64>,
    sx      : Vec<f64>,
    skc     : Vec<StaKey>,
    xc      : Vec<f64>,
    sc      : Vec<f64>,
}

impl Solution {
    pub fn new(tp : SolType,
               prosta    : i32,
               solsta    : i32,
               numvar    : usize,
               numcon    : usize) -> Solution {
        let prosta = match prosta {
            super::MSK_PRO_STA_DUAL_FEAS   => ProblemStatus::DualFeasible,
            super::MSK_PRO_STA_DUAL_INFEAS => ProblemStatus::DualInfeasible,
            super::MSK_PRO_STA_ILL_POSED   => ProblemStatus::Illposed,
            super::MSK_PRO_STA_PRIM_AND_DUAL_FEAS       => ProblemStatus::PrimalAndDualFeasible,
            super::MSK_PRO_STA_PRIM_AND_DUAL_INFEAS     => ProblemStatus::PrimalAndDualInfeasible,
            super::MSK_PRO_STA_PRIM_FEAS   => ProblemStatus::PrimalFeasible,
            super::MSK_PRO_STA_PRIM_INFEAS => ProblemStatus::PrimalInfeasible,
            super::MSK_PRO_STA_PRIM_INFEAS_OR_UNBOUNDED => ProblemStatus::PrimalInfeasibleOrUnbounded,
            super::MSK_PRO_STA_UNKNOWN     => ProblemStatus::Unknown,
            _                              => ProblemStatus::Unknown,
        };
        let (psta,dsta) = match solsta {
            super::MSK_SOL_STA_UNKNOWN            => (SolutionStatus::Unknown,SolutionStatus::Unknown),
            super::MSK_SOL_STA_OPTIMAL            => (SolutionStatus::Optimal,SolutionStatus::Optimal),
            super::MSK_SOL_STA_PRIM_FEAS          => (SolutionStatus::Feasible,SolutionStatus::Unknown),
            super::MSK_SOL_STA_DUAL_FEAS          => (SolutionStatus::Unknown,SolutionStatus::Feasible),
            super::MSK_SOL_STA_PRIM_AND_DUAL_FEAS => (SolutionStatus::Feasible,SolutionStatus::Feasible),
            super::MSK_SOL_STA_PRIM_ILLPOSED_CER  => (SolutionStatus::Unknown,SolutionStatus::IllposedCertificate),
            super::MSK_SOL_STA_PRIM_INFEAS_CER    => (SolutionStatus::Unknown,SolutionStatus::InfeasibilityCertificate),
            super::MSK_SOL_STA_DUAL_ILLPOSED_CER  => (SolutionStatus::IllposedCertificate,SolutionStatus::Unknown),
            super::MSK_SOL_STA_DUAL_INFEAS_CER    => (SolutionStatus::InfeasibilityCertificate,SolutionStatus::Unknown),
            super::MSK_SOL_STA_INTEGER_OPTIMAL    => (SolutionStatus::Optimal,SolutionStatus::Undefined),
            _                                     => (SolutionStatus::Unknown,SolutionStatus::Unknown),
        };

        return Solution {
            tp   : tp,
            prosta : prosta,
            psolsta : psta,
            dsolsta : dsta,
            skx  : vec![StaKey::Unknown; numvar],
            xx   : vec![0.0; numvar ],
            sx   : vec![0.0; numvar ],
            skc  : vec![StaKey::Unknown; numcon],
            xc   : vec![0.0; numcon],
            sc   : vec![0.0; numcon]
        };
    }
}

pub struct MosekTask {
    _env        : super::Env,
    task        : super::Task,

    // varblock_ptr[i] defines the offset into var_map where block i starts
    varblock_ptr : Vec<usize>,
    // var_map[i] >= 0, maps to linear variable varmap[i]
    // var_map[i] <  0, maps to psd variable element barelm_map[varmap[i]]
    var_map      : Vec<i64>,
    // varblock_map[i] == (block,offset) means that variable element i in variable block [block] at offset [offset]
    varblock_map : Vec<(usize,usize)>,
    // conslack[i] == 0 means no slack
    // conslack[i] > 0, slack variable for constraint is varmap[conslack[i]-1]
    // barelm_map[i] == (barvarj,k) means that psd element i belongs to barvarj at offset k.
    barelm_map  : Vec<(i32,i32,i32)>,
    // barvar_ptr[i] defines where in barelm_map the elements for barvar i starts
    barvar_ptr  : Vec<usize>,

    conblock_ptr : Vec<usize>,
    conslack    : Vec<usize>,

    // baraij accounting
    baraij_ptrb : Vec<usize>,
    baraij_ptre : Vec<usize>,
    baraij_num : Vec<usize>,
    baraijs    : Vec<(i32,i32)>,

    // Solutions
    sols : Vec<Solution>,
}

impl ConicSolverAPI for MosekTask {
    fn create_var_block(&mut self,c : &Domain) -> BlockIndex { self.create_var_block(c) }
    fn get_var_block_size(&self, i : BlockIndex) -> usize { self.get_var_block_size(i) }
    fn get_var_block_indexes(& self, i : BlockIndex, res : & mut [ElementIndex]) { self.get_var_block_indexes(i,res) }
    //fn put_var_block_name(&mut self,i : BlockIndex, name : &str, shape : Option<&[usize]>, sp : Option<&[usize]>) { /* default: do nothing */ }
    //fn delete_var_block(& mut self, i : BlockIndex) { panic!("Deleting variables not supported"); }

    fn create_con_block(& mut self, c : &Domain) -> BlockIndex { self.create_con_block(c)}
    fn get_con_block_size(&self, i : BlockIndex) -> usize { self.get_con_block_size(i) }
    fn get_con_block_indexes(& self, i : BlockIndex, res : & mut [ElementIndex]) { self.get_con_block_indexes(i,res) }
    //fn put_con_block_name(name : &str, sp : Option<&[usize]>, shape : Option<&[usize]>) -> Result<(),String> { /* default: do nothing */ }
    //fn delete_con_block(& mut self, i : BlockIndex) -> Result<(),String> { panic!("Deleting variables not supported"); }

    // Objective
    fn put_objective_name(& mut self, name : &str){ self.put_objective_name(name) }
    fn put_objective_sense(& mut self, sense : ObjectiveSense) { self.put_objective_sense(sense) }
    fn put_objective(& mut self, subj : &[ElementIndex], cof : &[f64]) { self.put_objective(subj, cof) }

    // Matrix
    fn put_a_row_list(& mut self, subi : &[usize], ptr : &[usize], subj : &[usize], cof : &[f64]) { self.put_a_row_list(subi,ptr,subj,cof) }
    fn put_aij_list(& mut self, subi : &[usize], subj : &[usize], cof : &[f64]) { self.put_aij_list(subi,subj,cof) }

    // Solutions
    fn get_num_solutions(& self) -> usize { self.get_num_solutions() }
    fn get_problem_status(& self, solidx : usize) -> Option<ProblemStatus> { self.get_problem_status(solidx) }
    fn get_solution_status(& self, solidx : usize) -> Option<(SolutionStatus,SolutionStatus)> { self.get_solution_status(solidx) }
    fn get_solution_type(& self, solidx : usize) -> Option<SolType> { self.get_solution_type(solidx) }
    fn get_primal_var_solution(& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> { self.get_primal_var_solution(solidx, idxs,res) }
    fn get_dual_var_solution  (& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> { self.get_dual_var_solution  (solidx, idxs,res) }
    fn get_primal_con_solution(& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> { self.get_primal_con_solution(solidx, idxs,res) }
    fn get_dual_con_solution  (& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> { self.get_dual_con_solution  (solidx, idxs,res) }

    // Task
    fn write_task(&self, filename : &str) { self.write_task(filename) }

    fn solve(&mut self) { self.solve(); }
    fn solve_with(&mut self,conf : &str) { self.solve_with(conf); }

    fn set_log_printer<F>(&mut self, f : Option<F>) where F : 'static+Fn(&str){ self.set_log_printer(f) }
    fn set_callback<F>(&mut self, f : Option<F>) where F : 'static+Fn(&str) -> bool { self.set_callback(f) }
}

impl MosekTask {
    pub fn new(name : Option<&str>) -> MosekTask {
        let env  = super::Env::new().unwrap();
        let task = env.task().unwrap();

        match name {
            Some(name) => {
                task.put_task_name(name).unwrap();
                task.put_task_name(name).unwrap();
            }
            None => {}
        }
        MosekTask{
            _env        : env,
            task        : task,

            varblock_ptr : vec![0],
            var_map     : Vec::new(),
            varblock_map : Vec::new(),
            barelm_map  : Vec::new(),
            barvar_ptr  : vec![0],

            conslack    : Vec::new(),
            conblock_ptr : vec![0],

            baraij_ptrb  : Vec::new(),
            baraij_ptre  : Vec::new(),
            baraij_num  : Vec::new(),
            baraijs     : Vec::new(),
            // Solutions
            sols        : Vec::new(),
        }
    }

    pub fn append_vars(& mut self, bk : i32, b : &[f64]) -> (usize,i32) {
        let n = b.len();
        let firstvar = self.task.get_num_var().unwrap();
        self.task.append_vars(n as i32).unwrap();
        self.task.put_var_bound_slice(firstvar,firstvar+n as i32, vec![bk; n].as_slice(), b, b).unwrap();

        let blockidx = self.varblock_ptr.len()-1;
        for i in 0..n {
            self.var_map.push(firstvar as i64+n as i64);
            self.varblock_map.push((blockidx,i));
        }
        self.varblock_ptr.push(self.varblock_map.len());
        (blockidx,firstvar)
    }
    pub fn append_cons(& mut self, bk : i32, b : &[f64]) -> (usize,i32) {
        let n = b.len();
        let firstcon = self.task.get_num_con().unwrap();
        self.task.append_cons(n as i32).unwrap();
        self.task.put_con_bound_slice(firstcon,firstcon+n as i32, vec![bk; n].as_slice(), b, b).unwrap();

        let blockidx = self.conblock_ptr.len()-1;
        let num_ordered_baraij = self.baraij_ptre.last().and_then(|v| Some(*v)).unwrap_or_else(|| 0 as usize);
        for _ in 0..n {
            self.conslack.push(0);
            self.baraij_ptrb.push(num_ordered_baraij);
            self.baraij_ptre.push(num_ordered_baraij);
            self.baraij_num.push(0);
        }

        self.conblock_ptr.push(self.conslack.len());
        (blockidx,firstcon)
    }
    pub fn append_cones(&mut self, ct : i32, ncone : usize, conedim : usize,conepar : Option<&[f64]>) -> usize {
        let n = ncone*conedim;
        let (blockidx,firstvar) = self.append_vars(super::MSK_BK_FR, vec![0.0; n].as_slice());
        let firstcone = self.task.get_num_cone().unwrap();
        let idxs : Vec<i32> = (firstvar..firstvar+n as i32).collect();
        match conepar {
            None =>
                for i in 0..ncone {
                    self.task.append_cone(ct,0.0,&idxs[firstcone as usize+(i*conedim)..firstcone as usize+((i+1)*conedim)]).unwrap()
                },
            Some(conepar) =>
                for i in 0..ncone {
                    self.task.append_cone(ct,conepar[i],&idxs[firstcone as usize+(i*conedim)..firstcone as usize+((i+1)*conedim)]).unwrap()
                },
        }
        blockidx
    }

    pub fn append_psdvar_block(&mut self,n : usize, dim : usize) -> BlockIndex {
        //let numelm = dim * (dim+1)/2;
        let firstbarj = self.task.get_num_barvar().unwrap();

        let blockidx = self.varblock_ptr.len()-1;

        self.task.append_barvars(vec![dim as i32; n].as_slice()).unwrap();

        let mut elmi = 0;
        for k in 0..n {
            for i in 0..dim {
                for j in 0..i+1 {
                    self.var_map.push(-(1+self.barelm_map.len() as i64));
                    self.varblock_map.push((blockidx,elmi));
                    self.barelm_map.push((firstbarj+k as i32,i as i32,j as i32));
                    elmi += 1;
                }
            }
            self.barvar_ptr.push(self.barelm_map.len())
        }
        self.varblock_ptr.push(self.varblock_map.len());
        blockidx
    }
    pub fn create_var_block(&mut self,dom : &Domain) -> BlockIndex {
        let bi =
            match dom {
                Domain::Free{num}  => self.append_vars(super::MSK_BK_FR,vec![0.0; *num].as_slice()).0,
                Domain::Zero{num} => self.append_vars(super::MSK_BK_FX,vec![0.0; *num].as_slice()).0,
                Domain::Lower{bound} => self.append_vars(super::MSK_BK_LO,bound).0,
                Domain::Upper{bound} => self.append_vars(super::MSK_BK_UP,bound).0,
                Domain::Fixed{bound} => self.append_vars(super::MSK_BK_FX,bound).0,
                Domain::QuadCone{num,dim}          => self.append_cones(super::MSK_CT_QUAD,*num,*dim,None),
                Domain::RotatedQuadCone{num,dim}   => self.append_cones(super::MSK_CT_RQUAD,*num,*dim,None),
                Domain::ExponentialCone{num}       => self.append_cones(super::MSK_CT_PEXP,*num,3,None),
                Domain::DualExponentialCone{num}   => self.append_cones(super::MSK_CT_DEXP,*num,3,None),
                Domain::PowerCone{dim,par}       => self.append_cones(super::MSK_CT_PPOW,par.len(),*dim,Some(par)),
                Domain::DualPowerCone{dim,par}   => self.append_cones(super::MSK_CT_DPOW,par.len(),*dim,Some(par)),
                Domain::PSDCone{num,dim}           => self.append_psdvar_block(*num,*dim),
            };
        bi as BlockIndex
    }
    pub fn get_var_block_size(&self, i : BlockIndex) -> usize { self.varblock_ptr[i+1] - self.varblock_ptr[i] }
    pub fn get_var_block_indexes(& self, i : BlockIndex, res : & mut [ElementIndex] ) {
        slice_fill_from_iterator(res, self.varblock_ptr[i]..self.varblock_ptr[i+1])
    }

    fn create_con_block(& mut self, dom : &Domain) -> BlockIndex {
        let blockidx = self.conblock_ptr.len()-1;

        let (bi,slackbi) =
            match dom {
                Domain::Free{num}  => (self.append_cons(super::MSK_BK_FR,vec![0.0; *num].as_slice()).0,None),
                Domain::Zero{num}  => (self.append_cons(super::MSK_BK_FX,vec![0.0; *num].as_slice()).0,None),
                Domain::Lower{bound} => (self.append_cons(super::MSK_BK_LO,bound).0,None),
                Domain::Upper{bound} => (self.append_cons(super::MSK_BK_UP,bound).0,None),
                Domain::Fixed{bound} => (self.append_cons(super::MSK_BK_FX,bound).0,None),
                Domain::PSDCone{num,dim} => {
                    let coneelm = dim*(dim+1)/2;
                    let bi      = self.append_cons(super::MSK_BK_FX,vec![0.0; num*coneelm].as_slice()).0;
                    let slackbi = self.append_psdvar_block(*num,*dim);

                    // add PSD slack non-zeros
                    let mut symmats : Vec<i64> = vec![0; coneelm];

                    {
                        let mut subi : Vec<i32> = Vec::with_capacity(coneelm);
                        let mut subj : Vec<i32> = Vec::with_capacity(coneelm);
                        let mut cof  : Vec<f64> = Vec::with_capacity(coneelm);
                        let mut dims  : Vec<i32> = Vec::with_capacity(coneelm);
                        let mut nz   : Vec<i64> = Vec::with_capacity(coneelm);
                        for i in 0..*dim {
                            for j in 0..i+1 {
                                subi.push(i as i32);
                                subj.push(j as i32);
                                cof.push(1.0);
                                dims.push(*dim as i32);
                                nz.push(1);
                            }
                        }
                        self.task.append_sparse_sym_mat_list(dims.as_slice(),
                                                             nz.as_slice(),
                                                             subi.as_slice(),
                                                             subj.as_slice(),
                                                             cof.as_slice(),
                                                             symmats.as_mut_slice()).unwrap();
                    }

                    let elmi = self.conblock_ptr[bi] as i32;
                    for _k in 0..*num {
                        let barj = self.barelm_map[(-(self.var_map[slackbi]+1)) as usize].0;
                        for i in 0..coneelm {
                            self.task.put_bara_ij(elmi,barj,vec![symmats[i]].as_slice(),vec![-1.0].as_slice()).unwrap();
                        }
                    }

                    (bi,Some(slackbi))
                }
                dom @ _  => {
                    let slackbi = self.create_var_block(dom);
                    let n = self.varblock_ptr[slackbi+1]-self.varblock_ptr[slackbi];
                    (self.append_cons(super::MSK_BK_FX,vec![0.0; n].as_slice()).0,
                     Some(slackbi))
                }
            };

        let blocksize = self.get_con_block_size(bi);
        match slackbi {
            None => for _i in 0..blocksize { self.conslack.push(0); },
            Some(slackbi) => {
                for (i,j) in (self.conblock_ptr[bi]..self.conblock_ptr[bi+1]).zip(self.varblock_ptr[slackbi]..self.varblock_ptr[slackbi+1]) {
                    self.conslack[i] = j+1;
                }
            }
        }

        blockidx as BlockIndex
    }
    fn get_con_block_size(&self, i : BlockIndex) -> usize { self.conblock_ptr[i+1] - self.conblock_ptr[i] }
    fn get_con_block_indexes(& self, i : BlockIndex, res : & mut [usize]) {
        slice_fill_from_iterator(res, self.conblock_ptr[i]..self.conblock_ptr[i+1]);
    }

    // Objective
    fn put_objective_name(& mut self, name : &str) { self.task.put_obj_name(name).unwrap(); }
    fn put_objective_sense(& mut self, sense : ObjectiveSense) {
        match sense {
            ObjectiveSense::Minimize => self.task.put_obj_sense(super::MSK_OBJECTIVE_SENSE_MINIMIZE),
            ObjectiveSense::Maximize => self.task.put_obj_sense(super::MSK_OBJECTIVE_SENSE_MAXIMIZE),
        }.unwrap();
    }
    fn put_objective(& mut self, subj : &[ElementIndex], cof : &[f64]) {
        let mut perm : Vec<usize> = (0..subj.len()).collect();
        perm.sort_by_key(|i| self.var_map[subj[*i]]);
        let mut p1 = 0; while self.var_map[subj[perm[p1]]] < 0 { p1 += 1 }
        let pe = subj.len();
        // linear non-zeros
        let numvar = self.task.get_num_var().unwrap();
        let csubj : Vec<i32> = (0..self.task.get_num_var().unwrap()).collect();
        let mut cval  = vec![0.0 ; numvar as usize];
        for p in p1..pe {
            cval[self.var_map[subj[perm[p]]] as usize] += cof[perm[p]];
        }
        self.task.put_c_list(csubj.as_slice(),cval.as_slice()).unwrap();

        // psd non-zeros
        if p1 > 0 {
            let mut j = 0;
            let mut barelmi : Vec<i32> = Vec::new();
            let mut barelmj : Vec<i32> = Vec::new();
            let mut barelmc : Vec<f64> = Vec::new();
            while j < p1 {
                barelmi.clear();
                barelmj.clear();
                barelmc.clear();

                let (barj,ii,jj) = self.barelm_map[-(1+self.var_map[perm[j]]) as usize];
                barelmi.push(ii);
                barelmj.push(jj);
                barelmc.push(cof[perm[j]]);
                j += 1;

                while j < pe && barj == self.barelm_map[-(1+self.var_map[perm[j]]) as usize].0 {
                    let (_barj,ii,jj) = self.barelm_map[-(1+self.var_map[perm[j]]) as usize];
                    if self.var_map[perm[j]] == self.var_map[perm[j-1]] {
                        let last = barelmc.len()-1;
                        barelmc[last] += cof[perm[j]];
                    }
                    else {
                        barelmi.push(ii);
                        barelmj.push(jj);
                        barelmc.push(cof[perm[j]]);
                    }
                    j += 1;
                }

                let dimbarj = self.task.get_dim_barvar_j(barj).unwrap();
                let mati = self.task.append_sparse_sym_mat(dimbarj,barelmi.as_slice(),barelmj.as_slice(),barelmc.as_slice()).unwrap();
                self.task.put_barc_j(barj,vec![mati].as_slice(),vec![1.0].as_slice()).unwrap();
            }

        }
    }

    // Matrix
    fn put_a_row_list(& mut self, subi : &[ElementIndex], ptr : &[usize], subj : &[ElementIndex], cof : &[f64]) {
        let nrows = subi.len();
        let mut perm : Vec<usize> = (0..subj.len()).collect();
        for i in 0..nrows {
            perm[ptr[i]..ptr[i+1]].sort_by_key(|j| self.var_map[subj[*j]]);
        }

        self.clear_baraijs(subi);
        let rsubi : Vec<i32> = subi.iter().map(|i| *i as i32).collect();
        let mut rsubj : Vec<i32> = Vec::new();
        let mut rcof  : Vec<f64> = Vec::new();
        let mut rptr  : Vec<i64> = vec![0; nrows+1];

        {
            let mut nzi = 0;
            // sort by internal indexes and merge duplicates
            for i in 0..nrows {
                let mut b = ptr[i];
                let end = ptr[i+1];
                while b < end && self.var_map[subj[perm[b]]] < 0 { b += 1; }

                if b < end {
                    rsubj.push(self.var_map[subj[perm[b]]] as i32);
                    rcof.push(cof[perm[b]]);
                    nzi += 1;
                    for _ in b+1 .. end {
                        if self.var_map[subj[perm[b]]] as i32 == rsubj[nzi-1] {
                            rcof[nzi-1] += cof[perm[b]];
                        }
                        else {
                            rsubj.push(self.var_map[subj[perm[b]]] as i32);
                            rcof.push(cof[perm[b]]);
                            nzi += 1;
                        }
                    }
                }
                rptr[i+1] = nzi as i64;
            }
        }

        self.task.put_a_row_list(rsubi.as_slice(),
                                 &rptr[0..nrows],
                                 &rptr[1..nrows+1],
                                 rsubj.as_slice(),
                                 rcof.as_slice() ).unwrap();


        for i in 0..nrows {
            let pb = ptr[i];
            let pe = { let mut r = pb; while r < ptr[i+1] && self.var_map[subj[r]] < 0 { r += 1; }; r };

            let mut j = pb;
            let mut barelmi : Vec<i32> = Vec::new();
            let mut barelmj : Vec<i32> = Vec::new();
            let mut barelmc : Vec<f64> = Vec::new();
            while j < pe {
                barelmi.clear();
                barelmj.clear();
                barelmc.clear();

                let (barj,ii,jj) = self.barelm_map[-(1+self.var_map[perm[j]]) as usize];
                barelmi.push(ii);
                barelmj.push(jj);
                barelmc.push(cof[perm[j]]);
                j += 1;

                while j < pe && barj == self.barelm_map[-(1+self.var_map[perm[j]]) as usize].0 {
                    let (_barj,ii,jj) = self.barelm_map[-(1+self.var_map[perm[j]]) as usize];
                    if self.var_map[perm[j]] == self.var_map[perm[j-1]] {
                        let last = barelmc.len()-1;
                        barelmc[last] += cof[perm[j]];
                    }
                    else {
                        barelmi.push(ii);
                        barelmj.push(jj);
                        barelmc.push(cof[perm[j]]);
                    }
                    j += 1;
                }

                let dimbarj = self.task.get_dim_barvar_j(barj).unwrap();
                let mati = self.task.append_sparse_sym_mat(dimbarj,barelmi.as_slice(),barelmj.as_slice(),barelmc.as_slice()).unwrap();
                self.task.put_bara_ij(subi[i] as i32,barj,vec![mati].as_slice(),vec![1.0].as_slice()).unwrap();
                self.baraijs.push((subi[i] as i32,barj));
                self.baraij_num[subi[i] as usize] += 1;
            }
        }
    }

    fn flush_baraijs(& mut self) {
        let numrow = self.baraij_ptrb.len();
        let num_ordered_baraij = self.baraij_ptre.last().and_then(|v| Some(*v)).unwrap_or_else(|| 0);
        if numrow > 0 && num_ordered_baraij < self.baraijs.len() {
            self.baraijs.sort();
            let mut si = 1;
            let mut di = 0;
            while si < self.baraijs.len() {
                if self.baraijs[si] == self.baraijs[di] {
                    si += 1;
                }
                else if di+1 < si {
                    di += 1;
                    self.baraijs[di] = self.baraijs[si];
                    si += 1;
                } else {
                    di += 1;
                    si += 1;
                }
            }
            self.baraijs.truncate(di+1);

            let mut i = 0;
            for p in 0..numrow {
                self.baraij_ptrb[i] = i;
                while i < self.baraijs.len() && self.baraijs[i].0 == p as i32 { i += 1 }
                self.baraij_ptre[i] = i;
            }

            for i in 0..self.baraij_num.len() {
                self.baraij_num[i] = self.baraij_ptre[i]-self.baraij_ptrb[i];
            }
        }
    }

    fn clear_baraijs(& mut self, subi : &[ElementIndex]) {
        if subi.iter().any(|i| self.baraij_num[*i] > 0) {
            self.flush_baraijs();

            let midx : Vec<i64> = Vec::new();
            let alpha : Vec<f64> = Vec::new();

            for i in subi.iter() {
                for j in self.baraij_ptrb[*i]..self.baraij_ptre[*i] {
                    let (i,j) = self.baraijs[j];
                    self.task.put_bara_ij(i,j,midx.as_slice(),alpha.as_slice()).unwrap();
                }
                self.baraij_num[*i] = 0;
                self.baraij_ptre[*i] = self.baraij_ptrb[*i];
            }
        }
    }

    fn put_aij_list(& mut self, subi : &[ElementIndex], subj : &[ElementIndex], cof : &[f64]) {
        let n = subi.len();
        let subj : Vec<i64> = subj.iter().map(|i| self.var_map[*i]).collect();
        let mut perm : Vec<usize> = (0..n).collect();
        perm.sort_by_key(|i| (subi[*i],self.var_map[subj[*i] as usize]));

        let mut i = 0;
        let mut asubi : Vec<i32> = Vec::new();
        let mut asubj : Vec<i32> = Vec::new();
        let mut acof  : Vec<f64> = Vec::new();

        let mut barilist : Vec<i32> = Vec::new();
        let mut barjlist : Vec<i32> = Vec::new();
        let mut bardim  : Vec<i32> = Vec::new();
        let mut barnz   : Vec<i64> = Vec::new();
        let mut barsubi : Vec<i32> = Vec::new();
        let mut barsubj : Vec<i32> = Vec::new();
        let mut barcof  : Vec<f64> = Vec::new();
        while i < n {
            let i0 = i;
            let conidx = subi[perm[i]]; i += 1;
            while i < n && conidx == subi[perm[i]] && subj[perm[i]] < 0 { i += 1 }
            let i1 = i;
            while i < n && conidx == subi[perm[i]] { i += 1 }
            let i2 = i;

            for j in i1..i2 {
                asubi.push(subi[perm[j]] as i32);
                asubj.push(subj[perm[j]] as i32);
                acof.push(cof[perm[j]]);
            }

            // PSD terms
            if i0 < i1 {
                let mut p = i0;
                while p < i1 {
                    let p0 = p;
                    let barj = self.barelm_map[(-subj[perm[p]]-1) as usize].0;
                    p += 1;
                    while barj == self.barelm_map[(-subj[perm[p]]-1) as usize].0 { p += 1 }
                    let p1 = p;

                    for _ in p0..p1 {
                        let (_,ii,jj) = self.barelm_map[(-subj[perm[p]]-1) as usize];
                        barsubi.push(ii);
                        barsubj.push(jj);
                        barcof.push(cof[perm[p]]);
                    }
                    bardim.push(self.task.get_dim_barvar_j(barj).unwrap());
                    barnz.push((p1-p0) as i64) ;
                    barjlist.push(barj);
                    barilist.push(conidx as i32);
                }
            }
        }

        if barjlist.len() > 0 {
            let mut symmats : Vec<i64> = vec![0; barjlist.len()];
            self.task.append_sparse_sym_mat_list(bardim.as_slice(),
                                                 barnz.as_slice(),
                                                 barsubi.as_slice(),
                                                 barsubj.as_slice(),
                                                 barcof.as_slice(),
                                                 symmats.as_mut_slice()).unwrap();
            let alpha = vec![1.0];
            let mut midx : Vec<i64> = vec![0];
            for i in 0..symmats.len() {
                midx[0] = symmats[i];
                self.task.put_bara_ij(barilist[i] as i32,
                                      barjlist[i] as i32,
                                      midx.as_slice(),
                                      alpha.as_slice()).unwrap();
                self.baraijs.push((barilist[i] as i32, barjlist[i] as i32));
                self.baraij_num[barilist[i] as usize] += 1;
            }
        }
        self.task.put_aij_list(asubi.as_slice(),asubj.as_slice(),acof.as_slice()).unwrap();
        
    }

    // Solutions
    fn get_sol(&self,solidx : usize) -> Option<&Solution> { if solidx < self.sols.len() { Some(&self.sols[solidx]) } else { None } }
    pub fn get_num_solutions  (& self) -> usize { self.sols.len() }
    pub fn get_problem_status (& self, solidx : usize) -> Option<ProblemStatus> { self.get_sol(solidx).and_then(|sol| Some(sol.prosta)) }
    pub fn get_solution_status(& self, solidx : usize) -> Option<(SolutionStatus,SolutionStatus)> { self.get_sol(solidx).and_then(|sol| Some((sol.psolsta,sol.dsolsta))) }
    pub fn get_solution_type  (& self, solidx : usize) -> Option<SolType> { self.get_sol(solidx).and_then(|sol| Some(sol.tp) ) }
    pub fn get_primal_var_solution(& self, solidx : usize, idxs : &[ElementIndex], res : & mut [f64]) -> Result<(),String> {
        match self.get_sol(solidx) {
            Some(sol) => {
                for (i,idx) in idxs.iter().enumerate() {
                    res[i] = sol.xx[*idx as usize];
                }
                Ok(())
            },
            None => Err("Solution not available".to_string()),
        }
    }
    pub fn get_dual_var_solution  (& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> {
        match self.get_sol(solidx) {
            Some(sol) => {
                for (i,idx) in idxs.iter().enumerate() {
                    res[i] = sol.sx[*idx as usize];
                }
                Ok(())
            },
            None => Err("Solution not available".to_string()),
        }
    }
    pub fn get_primal_con_solution(& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> {
        match self.get_sol(solidx) {
            Some(sol) => {
                for (i,idx) in idxs.iter().enumerate() {
                    res[i] = sol.xc[*idx as usize];
                }
                Ok(())
            },
            None => Err("Solution not available".to_string()),
        }
    }
    pub fn get_dual_con_solution  (& self, solidx : usize, idxs : &[ElementIndex], res : &mut [f64]) -> Result<(),String> {
        match self.get_sol(solidx) {
            Some(sol) => {
                for (i,idx) in idxs.iter().enumerate() {
                    res[i] = sol.sc[*idx as usize];
                }
                Ok(())
            },
            None => Err("Solution not available".to_string()),
        }
    }

    // Task
    pub fn put_task_name(& mut self, name : &str) { self.task.put_task_name(name).unwrap(); }
    pub fn write_task(&self, filename : &str) { self.task.write_data(filename).unwrap(); }


    fn solve_postprocess(&mut self) {
        self.sols.clear();
        let numvar = self.task.get_num_var().unwrap() as usize;
        let numcon = self.task.get_num_con().unwrap() as usize;
        //let numbarvar = self.task.get_num_barvar().unwrap() as usize;
        let numbarelm = self.barelm_map.len();

        let mut skx  = vec![0i32; numvar ];
        let mut xx   = vec![0.0; numvar ];
        let mut slx  = vec![0.0; numvar ];
        let mut sux  = vec![0.0; numvar ];
        let mut snx  = vec![0.0; numvar ];
        let mut skc  = vec![0i32; numcon ];
        let mut xc   = vec![0.0; numcon ];
        let mut y    = vec![0.0; numcon ];
        let mut barx = vec![0.0; numbarelm ];

        for which in vec![super::MSK_SOL_ITG,super::MSK_SOL_BAS,super::MSK_SOL_ITR].iter() {
            let which = *which;
            if self.task.solution_def(which).unwrap() {
                let mut sol = Solution::new(SolType::Integer,
                                            self.task.get_pro_sta(which).unwrap(),
                                            self.task.get_sol_sta(which).unwrap(),
                                            numvar+numbarelm,numcon );

                match sol.psolsta {
                    SolutionStatus::Undefined => {},
                    _ => {
                        self.task.get_xx (which,& mut xx[0..numvar]).unwrap();
                        self.task.get_skx(which,& mut skx[0..numvar]).unwrap();
                        self.task.get_xc (which,& mut xc[0..numcon]).unwrap();
                        self.task.get_skc(which,& mut skc[0..numcon]).unwrap();
                        if numbarelm > 0 { self.task.get_barx_slice(which, 0, numvar as i32, numbarelm as i64, barx.as_mut_slice()).unwrap(); }

                        // Copy primal variable solution values
                        for (i,idx) in self.var_map.iter().enumerate() {
                            if *idx < 0 {
                                sol.xx[i] = barx[-(*idx+1) as usize];
                                sol.skx[i] = StaKey::Unknown;
                            }
                            else {
                                sol.xx[i] = xx[*idx as usize];
                                sol.skx[i] = sk_to_stakey(skx[*idx as usize]);
                            }
                        }

                        // Copy primal constraint solution values
                        for (i,slacki) in self.conslack.iter().enumerate() {
                            if *slacki == 0 {
                                sol.xc[i]  = xc[i];
                                sol.skc[i] = sk_to_stakey(skc[i]);
                            }
                            else {
                                let idx = self.var_map[*slacki-1];
                                if idx < 0 {
                                    sol.xc[i]  = barx[-(idx+1) as usize];
                                    sol.skc[i] = StaKey::Unknown;
                                }
                                else {
                                    sol.xc[i]  = xx[idx as usize];
                                    sol.skc[i] = sk_to_stakey(skx[idx as usize]);
                                }
                            }
                        }
                    }
                }

                match sol.dsolsta {
                    SolutionStatus::Undefined => {},
                    _ => {
                        self.task.get_slx(which,& mut slx[0..numvar]).unwrap();
                        self.task.get_sux(which,& mut sux[0..numvar]).unwrap();
                        self.task.get_snx(which,& mut snx[0..numvar]).unwrap_or_else(|_| { snx.iter_mut().for_each(|v| *v = 0.0); });
                        self.task.get_y  (which,& mut y[0..numcon]).unwrap();
                        if numbarelm > 0 { self.task.get_bars_slice(which, 0, numvar as i32, numbarelm as i64, barx.as_mut_slice()).unwrap(); }

                        // Copy dual variable solution values
                        for (i,idx) in self.var_map.iter().enumerate() {
                            sol.sx[i] = {
                                if *idx < 0 { barx[-(*idx+1) as usize] }
                                else {
                                    let idx = *idx as usize;
                                    slx[idx]-sux[idx]+snx[idx]
                                }
                            };
                        }

                        // Copy primal constraint solution values
                        for (i,slacki) in self.conslack.iter().enumerate() {
                            sol.sc[i] = {
                                if *slacki == 0 { y[i] }
                                else {
                                    let idx = self.var_map[*slacki-1];
                                    if idx < 0 { barx[-(idx+1) as usize] }
                                    else {
                                        let idx = idx as usize;
                                        slx[idx]-sux[idx]+snx[idx]
                                    }
                                }
                            }
                        }
                    }
                }

                self.sols.push(sol);
            }
        }
    }

    pub fn solve(&mut self) {
        let _trm = self.task.optimize().unwrap();
        self.solve_postprocess();
    }
    pub fn solve_with(&mut self,conf : &str) {
        let (server,token) =
            match conf.find(';') {
                None => (conf,""),
                Some(p) => (&conf[0..p],&conf[p+1..conf.len()]),
            };
        let _trm = self.task.optimize_rmt(server,token).unwrap();
        self.solve_postprocess();
    }

    pub fn set_log_printer<F>(&mut self, f : Option<F>) where F : 'static+Fn(&str) {
        match f {
            Some(fun) => self.task.put_stream_callback(super::MSK_STREAM_LOG,fun),
            None    => self.task.clear_stream_callback(super::MSK_STREAM_LOG),
        }.unwrap();
    }

    pub fn set_callback<F>(&mut self, fun : Option<F>)
    where F : 'static+Fn(&str) -> bool
    {
        match fun {
            Some(fun) => self.task.put_callback(move |k,_,_,_| { let r = format!("{}",k); fun(r.as_str()) }).unwrap(),
            None => self.task.put_callback(|_,_,_,_| true).unwrap(),
        }
    }
}


// utilities

fn slice_fill_from_iterator<I:Iterator>(dst : &mut [I::Item], src : I) { dst.iter_mut().zip(src).for_each(|(d,s)| *d = s ) }


fn sk_to_stakey(sk : i32) -> StaKey {
    match sk {
        super::MSK_SK_BAS    => StaKey::Basic,
        super::MSK_SK_FIX    => StaKey::Fixed,
        super::MSK_SK_INF    => StaKey::Infinity,
        super::MSK_SK_LOW    => StaKey::AtBound,
        super::MSK_SK_SUPBAS => StaKey::SuperBasic,
        super::MSK_SK_UPR    => StaKey::AtBound,
        super::MSK_SK_UNK    => StaKey::Unknown,
        _             => StaKey::Unknown,
    }
}
