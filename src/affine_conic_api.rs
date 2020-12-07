use std::slice::SliceIndex;

// IndexManager is a structure that divides a range into a number of
// linked lists. Initially all elements are in the free list. When a
// block is allocated, elements are removed from (the beginning of)
// the free list, when block is released it is appended to (the end
// of) the free list.
//
// A Block is identified by an index to the first element is the
// block.
//
// Internal structure:
//   `link` defines the next and previous elements. For element i,
//     - If link[2*i]<0 then element i is the last in the block, otherwise next(i) = link[2*i]
//     - If link[2*i+1]<0 then element i is the first in the block, otherwise prev(i) = link[2*i+1]
//     If (link[2*i+1] == -2), then i is the head of a block (except for the free block)
#[derive(Debug)]
pub struct IndexManager {
    n          : usize, // capacity
    link       : Vec<isize>,
    first_free : isize, // index of the first element in the free block, -1 if empty
    last_free  : isize, // index of the last element in the free block, -1 if empty
    nfree      : usize  // current number of free elements
}

// impl fmt::Display for IndexManager {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"IndexManager{"

//         write!(f, "({}, {})", self.x, self.y)
//     }
// }

struct KeyGenerator<'a> {
    shape : & 'a [usize],
    index : Vec<usize>
}

impl<'a> KeyGenerator<'a> {
    fn new<'b>(shape :  & 'b [usize]) -> KeyGenerator<'b> { KeyGenerator{ shape : shape, index : vec![0;shape.len()]} }
    fn inc(& mut self) {
        let mut i = self.shape.len();
        if self.index.len() > 0 {
            self.index[i-1] += 1;
            while i > 1 && self.index[i-1] == self.shape[i-1] {
                self.index[i-1] = 0;
                self.index[i-2] += 1;
                i -= 1;
            }
        }
    }
    fn get(& self) -> &[usize] { self.index.as_slice() }
    fn append_to_str(& self, s : & mut String) {
        if self.index.len() > 0 {
            s.push_str(format!("{}",self.index[0]).as_str());
            for i in self.index[1..].iter() {
                s.push_str(format!(",{}",*i).as_str());
            }
        }
    }
}


impl IndexManager {
    pub fn validate(&self) -> Result<(),String> {
        if self.n*2 != self.link.len() { return Err("Invalid link length".to_string()); }
        if self.link[self.first_free as usize*2+1] >= 0 { return Err("Inconsistent first free".to_string()); }
        if self.link[self.last_free as usize*2] >= 0 { return Err("Inconsistent last free".to_string()); }
        Ok(())
    }

    pub fn new(n : usize) -> IndexManager {
        let mut link = vec![0; 2*n];
        unsafe {
            *link.get_unchecked_mut(1)       = -1 as isize; // prev[0] = -1
            *link.get_unchecked_mut(2*(n-1)) = -1 as isize; // next[n-1] = -1
            for i in 0..n-1 {
                *link.get_unchecked_mut(2*i) = (i+1)   as isize; // next[i] = i+1
                *link.get_unchecked_mut(2*(i+1)+1) = i as isize; // prev[i+1] = i
            }
        }

        IndexManager{ n : n, link:link, first_free : 0, last_free  : (n-1) as isize, nfree : n }
    }

    pub fn allocate(& mut self, n : usize) -> usize {
        if self.nfree == 0 {
            self.link.resize((self.n+n)*2,-1 as isize);
            unsafe {
                for i in self.n .. (self.n+n-1) {
                    *self.link.get_unchecked_mut(2*i)       = (i+1) as isize;
                    *self.link.get_unchecked_mut(2*(i+1)+1) = i as isize;
                }
            }

            self.first_free = self.n as isize;
            self.last_free  = (self.n+n-1) as isize;
            self.nfree += n;
            self.n += n;
        }
        else if self.nfree < n {
            let addn = n-self.nfree;
            self.link.resize((self.n+addn)*2,-1 as isize);
            unsafe {
                for i in self.n .. self.n+addn-1 {
                    *self.link.get_unchecked_mut(2*i) = (i+1) as isize;
                    *self.link.get_unchecked_mut(2*(i+1)+1) = i as isize;
                }
                *self.link.get_unchecked_mut(2*self.n+1) = self.last_free;
            }
            self.last_free = (self.n+addn-1) as isize;
            self.nfree += addn;
            self.n += addn;
        }

        let head = self.first_free as usize;
        let mut tail = head;

        unsafe {
            for _k in 0..n-1 { tail = *self.link.get_unchecked_mut(tail*2) as usize; }
        }

        self.nfree -= n;

        if self.link[tail*2] < 0 {
            self.last_free = -1 as isize;
            self.first_free = -1 as isize;
        }
        else {
            self.first_free = self.link[tail*2]; // first_free = next[tail]
            self.link[tail*2] = -1; // next[tail] = -1
            self.link[self.first_free as usize * 2 + 1] = -1; // prev[first_free] previous if free[first] = -1;
        }

        self.link[head*2+1] = -((n+1) as isize);

        head
    }

    // Free the block pointed to by head, placing it at the end of the
    // current free list
    pub fn release(& mut self, head : usize) {
        if head >= self.n || self.link[head*2+1] > -2 {
            panic!("Index is not head of an allocated block");
        }

        let blocksize = (-self.link[head*2+1]-1) as usize;
        let mut tail = head;
        let mut n : usize = 1;
        unsafe {
            while *self.link.get_unchecked(tail*2) >= 0 {
                tail = *self.link.get_unchecked(tail*2) as usize;
                n += 1;
            }
        }
        if blocksize != n {
            panic!("Internal inconsistency in linked list");
        }


        if self.first_free < 0 { // currently no free
            self.first_free = head as isize;
            self.last_free  = tail as isize;
        }
        else {
            self.link[self.first_free as usize * 2 + 1] = tail as isize; // prev[first_free] = tail
            self.link[tail as usize * 2] = self.first_free; // next[tail] = first_free;
            self.first_free = head as isize;
        }

        self.nfree += n;
    }

    pub fn capacity(& self) -> usize { self.n }
    pub fn blockmaxindex(&self, p : usize) -> usize {
        let mut i = p as isize;
        let mut maxi = i;
        while i >= 0 {
            if i > maxi { maxi = i; }
            i = self.link[(i as usize)*2];
        }
        return maxi as usize;
    }

    pub fn get(&self, head : usize, idxs : & mut [usize]) {
        if head >= self.n || self.link[head*2+1] > -2 {
            panic!("Index is not head of an allocated block");
        }

        let blocksize = (-self.link[head*2+1]-1) as usize;
        if idxs.len() < blocksize {
            panic!("index out of bounds in idxs");
        }

        unsafe {
            let mut p = head;
            for i in 0..blocksize {
                *idxs.get_unchecked_mut(i) = p;
                p = *self.link.get_unchecked(p*2) as usize;
            }
        }
    }
}

pub struct BlockManager {
    idxs : IndexManager,
    blks : IndexManager,
    heads : Vec<usize>
}

impl BlockManager {
    pub fn new() -> BlockManager {
        BlockManager{
            idxs : IndexManager::new(128),
            blks : IndexManager::new(128),
            heads : vec![0;128]
        }
    }

    pub fn allocate(& mut self, n : usize) -> usize {
        let id = self.blks.allocate(1);
        let blockid = self.idxs.allocate(n);

        if self.blks.capacity() > self.heads.len() {
            self.heads.resize(self.blks.capacity(),0);
        }

        self.heads[id] = blockid;
        return id;
    }

    pub fn release(& mut self, id : usize) {
        self.blks.release(id);
        self.idxs.release(self.heads[id]);
    }

    pub fn block_capacity(&self) -> usize { self.heads.len() }
    pub fn capacity(&self) -> usize { self.idxs.capacity() }


    pub fn get(&self, blockid : usize,  idxs : & mut [usize] ) { self.idxs.get(self.heads[blockid],idxs) }
    pub fn blockmaxindex(&self, blockid : usize) -> usize { self.idxs.blockmaxindex(self.heads[blockid]) }
}


#[derive(Copy,Clone)]
struct MaybeUsize  { value : usize }
impl MaybeUsize {
    fn some(v : usize) -> MaybeUsize { MaybeUsize{value : v+1} }
    fn none() -> MaybeUsize {  MaybeUsize {value : 0 } }
    fn get(&self) -> Option<usize> {
        return
            if self.value > 0 {
                Some(self.value-1)
            }
        else {
            None
        }
    }
}



pub enum DomainType {
    R,
    RMinus,
    RPlus,
    RZero,
    QuadraticCone,
    RotatedQuadraticCone,
    PrimalExpCone,
    DualExpCone,
    PrimalPowerCone,
    DualPowerCone,
    PrimalGeometricMeanCone,
    DualGeometricMeanCone,
    InfNormCone,
    OneNormCone,
    PSDCone
}

pub struct Domain {
    dt        : DomainType,
    shape     : Option<Vec<usize>>,
    conedim   : Option<usize>,
    rhs       : Option<Vec<f64>>,
    par       : Option<Vec<f64>>
}

impl Domain {
    fn new(dt : DomainType) -> Domain           { Domain{ dt : dt, shape : None, conedim : None, rhs : None, par : None } }
    fn new_with_rhs(dt : DomainType, rhs : &[f64]) -> Domain { Domain{ dt : dt, shape : None, conedim : None, rhs : Some(rhs.to_vec()), par : None } }
    pub fn r() -> Domain               { Domain::new(DomainType::R) }
    pub fn r_minus() -> Domain         { Domain::new(DomainType::RMinus) }
    pub fn r_plus() -> Domain          { Domain::new(DomainType::RPlus) }
    pub fn r_zero() -> Domain          { Domain::new(DomainType::RZero) }
    pub fn less_than(rhs : &[f64]) -> Domain    { Domain::new_with_rhs(DomainType::RMinus,rhs) }
    pub fn greater_than(rhs : &[f64]) -> Domain { Domain::new_with_rhs(DomainType::RPlus,rhs) }
    pub fn equals(rhs : &[f64]) -> Domain       { Domain::new_with_rhs(DomainType::RZero,rhs) }
    pub fn quadratic_cone() -> Domain  { Domain::new(DomainType::QuadraticCone) }
    pub fn rotated_quadratic_cone() -> Domain { Domain::new(DomainType::RotatedQuadraticCone) }
    pub fn primal_exp_cone() -> Domain          { Domain::new(DomainType::PrimalExpCone) }
    pub fn dual_exp_cone() -> Domain            { Domain::new(DomainType::DualExpCone) }
    pub fn primal_power_cone(par : &[f64]) -> Domain { Domain{dt : DomainType::PrimalPowerCone, shape : None, conedim : None, rhs : None, par : Some(par.to_vec()) } }
    pub fn dual_power_cone(par : &[f64]) -> Domain   { Domain{dt : DomainType::DualPowerCone,   shape : None, conedim : None, rhs : None, par : Some(par.to_vec()) } }
    pub fn primal_geometric_mean_cone() -> Domain { Domain::new(DomainType::PrimalGeometricMeanCone) }
    pub fn dual_geometric_mean_cone() -> Domain   { Domain::new(DomainType::DualGeometricMeanCone) }
    pub fn inf_norm_cone() -> Domain { Domain::new(DomainType::InfNormCone) }
    pub fn one_norm_cone() -> Domain { Domain::new(DomainType::OneNormCone) }
    pub fn psd_cone() -> Domain      { Domain::new(DomainType::PSDCone) }

    pub fn with_shape(mut self, shape : &[usize]) -> Domain {
        match self.shape {
            None => {
                match self.rhs {
                    None => self.shape = Some(shape.to_vec()),
                    Some(ref rhs) => {
                        if rhs.len() != shape.iter().product() {
                            panic!("Mismatching shape given")
                        }
                        else {
                            self.shape = Some(shape.to_vec())
                        }
                    }
                }
            }
            Some(ref shp) => {
                if shape.iter().product::<usize>() != shp.iter().product::<usize>() {
                    panic!("Mismatching shape given")
                }
                else {
                    self.shape = Some(shape.to_vec())
                }
            }
        }

        match self.conedim {
            None => {},
            Some(ref d) => {
                if *d >= shape.len() {
                    panic!("Cone dimension is out of bounds in given shape")
                }
            }

        }

        self
    }

    pub fn with_size(self, size : usize) -> Domain {
        self.with_shape(&[size])
    }

    pub fn with_conedim(mut self, conedim : usize) -> Domain {
        if conedim > 0 {
            match self.shape {
                Some(ref shp) => {
                    if shp.len() <= conedim {
                        panic!("Cone dimension is out of bounds")
                    }
                },
                None => {}
            }
        }

        self.conedim = Some(conedim);
        self
    }

    pub fn with_rhs(mut self, rhs : &[f64]) -> Domain {
        match self.rhs {
            None => match self.shape {
                None => {},
                Some(ref shp) => {
                    if shp.iter().product::<usize>() != rhs.len() {
                        panic!("Right hand side size does not match current shape")
                    }
                }
            }
            Some(ref rhs) => {
                if rhs.len() != rhs.len() {
                    panic!("Right hand side size does not match current shape")
                }
            }
        }
        self.rhs = Some(rhs.to_vec());
        self
    }

    pub fn unwrap(self) -> (DomainType,Vec<usize>,usize,Vec<f64>,Option<Vec<f64>>) {
        let (dt,optshape,optconedim,optrhs,optpar) = (self.dt,self.shape,self.conedim,self.rhs,self.par);

        let (shape,domsize,nd) : (Vec<usize>,usize,usize) =
            match optshape {
                Some(shp) => { let domsize = shp.iter().product(); let nd = shp.len(); (shp,domsize,nd) },
                None => match optrhs {
                    Some(ref rhs) => (vec![rhs.len()],rhs.len(),1usize),
                    None => {
                        match &dt {
                            PrimalExpCone => (vec![3],3usize,1usize),
                            DualExpCone   => (vec![3],3usize,1usize),
                            _ => { panic!("Unknown size of domain"); }
                        }
                    }
                }
            };

        let rhs =
            match optrhs {
                None => vec![0.0; domsize],
                Some(rhs) => rhs
            };

        let conedim =
            match self.conedim {
                None => nd-1,
                Some(conedim) => conedim
            };

        match dt {
            DomainType::R|DomainType::RMinus|DomainType::RPlus|DomainType::RZero => {},
            DomainType::QuadraticCone        => {
                if domsize < 1 { panic!("Invalid shape for this QuadraticCone domain"); }
                if optpar.is_some() { panic!("QuadraticCone does not accept parameters"); }
            },
            DomainType::RotatedQuadraticCone => {
                if domsize < 2 { panic!("Invalid shape for this RotatedQuadraticCone domain"); }
                if optpar.is_some() { panic!("RotatedQuadraticCone does not accept parameters"); }
            },
            DomainType::PrimalExpCone => {
                if domsize != 3 { panic!("Invalid shape for this PrimalExpCone domain"); }
                if optpar.is_some() { panic!("PrimalExpCone does not accept parameters"); }
            },
            DomainType::DualExpCone  => {
                if domsize != 3 { panic!("Invalid shape for this DualExpCone domain"); }
                if optpar.is_some() { panic!("DualExpCone does not accept parameters"); }
            },
            DomainType::PrimalPowerCone => {
                if domsize < 2 { panic!("Invalid shape for this PrimalPowerCone domain"); }
                match optpar {
                    None => panic!("PrimalPowerCone domain requires parameters"),
                    Some(ref par) => if par.len() > domsize { panic!("PrimalPowerCone parameter vector is too long") },
                }
            },
            DomainType::DualPowerCone => {
                if domsize < 2 { panic!("Invalid shape for this DualPowerCone domain"); }
                match optpar {
                    None => panic!("DualPowerCone domain requires parameters"),
                    Some(ref par) => if par.len() > domsize { panic!("DualPowerCone parameter vector is too long") },
                }
            }
            DomainType::PrimalGeometricMeanCone => {
                if domsize < 2 { panic!("Invalid shape for this PrimalGeometricMeanCone domain"); }
                if optpar.is_some() { panic!("PrimalGeometricMeanCone does not accept parameters"); }
            },
            DomainType::DualGeometricMeanCone => {
                if domsize < 2 { panic!("Invalid shape for this DualGeometricMeanCone domain"); }
                if optpar.is_some() { panic!("DualGeometricMeanCone does not accept parameters"); }
            }
            DomainType::InfNormCone => {
                if domsize < 1 { panic!("Invalid shape for this InfNormCone domain"); }
                if optpar.is_some() { panic!("InfNormCone does not accept parameters"); }
            }
            DomainType::OneNormCone => {
                if domsize < 1 { panic!("Invalid shape for this OneNormCone domain"); }
                if optpar.is_some() { panic!("OneNormCone does not accept parameters"); }
            }
            DomainType::PSDCone => {
                let conesize = shape[conedim];
                let d = ((((1+8*conesize) as f64).sqrt()-1.0)/2.0).floor() as usize;
                if d != conesize {
                    panic!("Invalid cone dimension for PSDCone domain");
                }
                if optpar.is_some() { panic!("PSDCone does not accept parameters"); }
            }
        }

        (dt,shape,conedim,rhs,optpar)
    }
}


pub struct Model {
    env  : super::Env,
    task : super::Task,

    model_name : Option<String>,

    con_blocks : BlockManager,
    afe_blocks : BlockManager,
    obj_blocks : BlockManager,

    acc_block_map  : BlockManager,
    acon_block_map : BlockManager,

    // maps aconid -> accid
    acon_acc : Vec<usize>,
    // maps aconid -> afeid
    acon_afe : Vec<usize>,

    // maps aconelmidx -> taskaccidx
    acon_elm_accid : Vec<usize>,
    // maps aconelmidx -> offset into ACC
    acon_elm_ofs   : Vec<usize>,
    // maps aconelmidx -> taskafeidx
    acon_elm_afe   : Vec<usize>,


    // Linear variables
    var_block_map : BlockManager,
    // maps varid -> accid. For k=var_block_acc_id[i], if k > 0 then
    // (k-1) is the id of the associated ACC block.
    var_block_afe_id : Vec<usize>,
    var_block_acc_id : Vec<usize>,
    // maps varelmidx -> ACC idx
    var_elm_acc_idx : Vec<usize>,
    // maps varelmidx -> ACC offset
    var_elm_acc_ofs : Vec<usize>,
    var_elm_afe     : Vec<usize>,

    barvar_block_map : BlockManager,
    barvar_ptr : Vec<usize>,
    barvar_elm_idx : Vec<usize>,
    barvar_elm_ofs : Vec<usize>
}

fn inv_tril_idx(d:usize,ofs:usize) -> (usize,usize) {
    let 2d_1 = (2*d+1) as f64;
    let j = (2d_1-(2d_1*2d_1-8.0*p as f64).sqrt()/2.0).floor() as usize;
    let i = ofs-j;

    (i,j)
}

impl Model {
    pub fn new() -> Model {
        let env : super::Env = super::Env::new().unwrap();
        let mut task : super::Task = env.task().unwrap();

        // zero sized domain
        let _dom0idx = task.append_r_domain(0).unwrap();

        Model {
            env  : env,
            task : task,
            model_name : None,
            con_blocks : BlockManager::new(),
            afe_blocks : BlockManager::new(),
            obj_blocks : BlockManager::new(),

            acc_block_map  : BlockManager::new(),
            acon_block_map : BlockManager::new(),

            // maps aconid -> accid
            acon_acc : Vec::new(),
            // maps aconid -> afeid
            acon_afe : Vec::new(),

            // maps aconelmidx -> taskaccidx
            acon_elm_accid : Vec::new(),
            // maps aconelmidx -> offset into ACC
            acon_elm_ofs   : Vec::new(),
            // maps aconelmidx -> taskafeidx
            acon_elm_afe   : Vec::new(),

            // Linear variables
            var_block_map : BlockManager::new(),
            var_block_afe_id : Vec::new(),
            // maps varid -> accid. For k=var_block_acc_id[i], if k > 0 then
            // (k-1) is the id of the associated ACC block.
            var_block_acc_id : Vec::new(),
            // maps varelmidx -> ACC idx
            var_elm_acc_idx : Vec::new(),
            // maps varelmidx -> ACC offset
            var_elm_acc_ofs : Vec::new(),
            var_elm_afe     : Vec::new(),

            barvar_block_map : BlockManager::new(),
            barvar_ptr : vec![0],
            barvar_elm_idx : Vec::new(),
            barvar_elm_ofs : Vec::new()
        }
    }

    fn var_allocate(& mut self, numcone : usize, conesize : usize) -> usize {
        let n = numcone*conesize;
        // Allocate ACCs
        let accid = self.acc_allocate(numcone);
        let mut accelmidxs : Vec<usize> = vec![0usize; numcone];
        self.acc_block_map.get(accid, accelmidxs.as_mut_slice());
        // Allocate AFEs
        let afeid = self.afe_allocate(n);
        let mut afeelmidxs : Vec<usize> = vec![0usize; n];
        self.afe_blocks.get(afeid, afeelmidxs.as_mut_slice());

        // Allocate variables
        let varid = self.var_block_map.allocate(n);
        {
            let minnumvar = self.var_block_map.blockmaxindex(varid)+1;
            let numvar = self.task.get_num_var().unwrap() as usize;
            if numvar < minnumvar {
                self.task.append_vars((minnumvar-numvar) as i32).unwrap();
            }

            if varid+1 > self.var_block_acc_id.len() {
                self.var_block_acc_id.resize(varid+1, 0);
                self.var_block_afe_id.resize(varid+1, 0);
            }

            if self.var_elm_acc_idx.len() < minnumvar {
                self.var_elm_acc_idx.resize(minnumvar,0);
                self.var_elm_acc_ofs.resize(minnumvar,0);
                self.var_elm_afe.resize(minnumvar,0);
            }
        }

        self.var_block_acc_id[varid] = accid;
        self.var_block_afe_id[varid] = afeid;

        let mut varelmidxs : Vec<usize> = vec![0usize; n];
        self.var_block_map.get(varid, varelmidxs.as_mut_slice());

        unsafe {
            let mut k : usize = 0;
            for i in 0..numcone {
                for j in 0..conesize {
                    let varelmidx : usize = *varelmidxs.get_unchecked(k);
                    *self.var_elm_acc_idx.get_unchecked_mut(varelmidx) = *accelmidxs.get_unchecked(i);
                    *self.var_elm_acc_ofs.get_unchecked_mut(varelmidx) = j;
                    *self.var_elm_afe.get_unchecked_mut(varelmidx)     = *afeelmidxs.get_unchecked(k);
                    k += 1;
                }
            }
        }

        let afeidxs : Vec<i64> = afeelmidxs.iter().map(|v| *v as i64).collect();
        let accidxs : Vec<i64>  = accelmidxs.iter().map(|v| *v as i64).collect();
        let nzrow   : Vec<i32> = vec![1; n];
        let ptrrow  : Vec<i64>  = (0_i64..n as i64).collect();
        let idxrow  : Vec<i32>= varelmidxs.iter().map(|v| *v as i32).collect();
        let cofrow = vec![1.0; n];

        self.task.put_afe_f_row_list(afeidxs.as_slice(),nzrow.as_slice(),ptrrow.as_slice(),idxrow.as_slice(),cofrow.as_slice());
        varid
    }

    fn barvar_allocate(& mut self, n : usize, conedim : usize) -> usize {
        let barvarid = self.barvar_block_map.allocate(n);
        let mut dims : Vec<i32> = Vec::new(); dims.resize(n,conedim as i32);
        let first = self.task.get_num_barvar().unwrap() as usize;
        let firstelm = self.barvar_elm_idx.len();
        let newnumbarvar = first+n;
        let newnumbarvarelm = firstelm+n*conedim*(conedim+1)/2;
        self.task.append_barvars(dims.as_slice()).unwrap();
        self.barvar_ptr.resize(newnumbarvar,0);
        self.barvar_elm_idx.resize(newnumbarvarelm,0);
        self.barvar_elm_ofs.resize(newnumbarvarelm,0);

        unsafe {
            let mut p = firstelm;
            for i in first..first+n {
                *self.barvar_ptr.get_unchecked_mut(i) = p;
                p += conedim*(conedim+1)/2;
            }
            let mut k = 0;
            for i in 0..n {
                for j in 0..conedim*(conedim+1)/2 {
                    *self.barvar_elm_idx.get_unchecked_mut(firstelm+k) = first+i;
                    *self.barvar_elm_ofs.get_unchecked_mut(firstelm+k) = j;
                    k += 1;
                }
            }
        }
        return barvarid;
    }

    fn con_allocate(& mut self, n : usize) -> usize {
        let conid     = self.con_blocks.allocate(n);
        let minnumcon = self.con_blocks.blockmaxindex(conid)+1;
        let numcon    = self.task.get_num_con().unwrap() as usize;

        if numcon < minnumcon {
            let addnumcon = minnumcon - numcon;
            self.task.append_cons(addnumcon as i32).unwrap();
        }
        conid
    }

    fn afe_allocate(& mut self, n : usize) -> usize {
        let afeid = self.afe_blocks.allocate(n);
        let minnumafe = self.afe_blocks.blockmaxindex(afeid)+1;
        let numafe    = self.task.get_num_afe().unwrap() as usize;

        if numafe < minnumafe {
            let addnumafe = minnumafe - numafe;
            self.task.append_afes(addnumafe as i64).unwrap();
        }
        afeid
    }

    fn acon_allocate(& mut self, numcone : usize, conesize : usize) -> usize {
        let n = numcone * conesize;

        let mut afeidxs = vec![0usize; n];
        let mut conidxs = vec![0usize; n];
        let mut accidxs = vec![0usize; numcone];

        let afeid = self.afe_allocate(n);
        let conid = self.acon_block_map.allocate(n);
        let accid = self.acc_allocate(numcone);
        self.afe_blocks.get(afeid,afeidxs.as_mut_slice());
        self.acon_block_map.get(conid,conidxs.as_mut_slice());
        self.acc_block_map.get(accid,accidxs.as_mut_slice());

        let acon_block_cap = self.acon_block_map.block_capacity();
        if self.acon_acc.len() < acon_block_cap {
            self.acon_acc.resize(acon_block_cap,0);
            self.acon_afe.resize(acon_block_cap,0);
        }
        let acon_cap = self.acon_block_map.block_capacity();
        if self.acon_elm_accid.len() < acon_cap {
            self.acon_elm_accid.resize(acon_cap,0);
            self.acon_elm_ofs.resize(acon_cap,0);
        }

        self.acon_acc[conid] = accid;
        self.acon_afe[conid] = afeid;

        unsafe {
            let mut  k = 0usize;
            for i in 0..numcone {
                let coneidx = *accidxs.get_unchecked(i);
                for j in 0..conesize {
                    *self.acon_elm_accid.get_unchecked_mut(k) = coneidx;
                    *self.acon_elm_ofs.get_unchecked_mut(k) = j;
                    k += 1;
                }
            }
        }

        conid
    }

    fn acc_allocate(& mut self, numcone : usize) -> usize {
        let accid = self.acc_block_map.allocate(numcone);
        let minnumacc = self.acc_block_map.blockmaxindex(accid)+1;
        let numacc    = self.task.get_num_acc().unwrap() as usize;
        if numacc < minnumacc {
            let domidxs : Vec<i64> = vec![0; minnumacc-numacc];
            let afeidxs : [i64;0] = [];
            let b       : [f64;0] = [];
            self.task.append_accs(domidxs.as_slice(),&afeidxs,&b).unwrap();
        }
        accid
    }

    //

    fn add_domain(& mut self, dt : DomainType, domsize : usize, par : Option<Vec<f64>>) -> Vec<i64> {
        let n = domsize as i64;
        match (dt,par) {
            (DomainType::R,_)                       => (vec![self.task.append_r_domain(domsize as i64).unwrap()]),
            (DomainType::RMinus,_)                  => (vec![self.task.append_rminus_domain(domsize as i64).unwrap()]),
            (DomainType::RPlus,_)                   => (vec![self.task.append_rminus_domain(domsize as i64).unwrap()]),
            (DomainType::RZero,_)                   => (vec![self.task.append_rminus_domain(domsize as i64).unwrap()]),
            (DomainType::QuadraticCone,_)           => (0..numcone).map(|_i| self.task.append_quadratic_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::RotatedQuadraticCone,_)    => (0..numcone).map(|_i| self.task.append_r_quadratic_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::PrimalExpCone,_)           => (0..numcone).map(|_i| self.task.append_primal_exp_cone_domain().unwrap()).collect(),
            (DomainType::DualExpCone,_)             => (0..numcone).map(|_i| self.task.append_dual_exp_cone_domain().unwrap()).collect(),
            (DomainType::PrimalGeometricMeanCone,_) => (0..numcone).map(|_i| self.task.append_primal_geo_mean_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::DualGeometricMeanCone,_)   => (0..numcone).map(|_i| self.task.append_dual_geo_mean_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::InfNormCone,_)             => (0..numcone).map(|_i| self.task.append_dual_geo_mean_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::OneNormCone,_)             => (0..numcone).map(|_i| self.task.append_one_norm_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::PSDCone,_)                 => (0..numcone).map(|_i| self.task.append_psd_cone_domain(conesize as i64).unwrap()).collect(),
            (DomainType::PrimalPowerCone,Some(ref par)) => (0..numcone).map(|_i| self.task.append_primal_power_cone_domain(conesize as i64, par.as_slice()).unwrap()).collect(),
            (DomainType::DualPowerCone,Some(ref par))   => (0..numcone).map(|_i| self.task.append_dual_power_cone_domain(conesize as i64,par.as_slice()).unwrap()).collect(),
            (DomainType::PrimalPowerCone,None)     => panic!("Missing parameter for cone"),
            (DomainType::DualPowerCone,Nonoe)      => panic!("Missing parameter for cone")
        }
    }

    pub fn variable(& mut self, name : Option<&str>, dom : Domain) -> Variable {
        let (dt,shape,conedim,rhs,par) = dom.unwrap();
        let domsize = shape.iter().product::<usize>();
        let conesize = shape[conedim];
        let numcone = domsize / conesize;
        let nd = shape.len();

        let mut varelmidx : Vec<usize> = vec![0; domsize];
        let mut accelmidx : Vec<usize> = vec![0; numcone];
        let mut afeelmidx : Vec<usize> = vec![0; domsize];
        let varid = self.var_allocate(numcone,conesize);
        let accid = self.var_block_acc_id[varid];
        let afeid = self.var_block_acc_id[varid];
        self.var_block_map.get(varid, varelmidx.as_mut_slice());
        self.acc_block_map.get(accid, accelmidx.as_mut_slice());
        self.afe_blocks.get(afeid, afeelmidx.as_mut_slice());

        {
            let accidxs : Vec<i64> = accelmidx.iter().map(|v| *v as i64).collect();
            let afeidxs : Vec<i64> = afeelmidx.iter().map(|v| *v as i64).collect();
            let varidxs : Vec<i32> = varelmidx.iter().map(|v| *v as i32).collect();

            let domidxs = self.add_domain(dt,domsize,par);
            self.task.put_acc_list(accidxs.as_slice(),domidxs.as_slice(),afeidxs.as_slice(),rhs.as_slice());
        }

        match name {
            Some(name) => {
                {
                    let mut varnamegen = KeyGenerator::new(shape.as_slice());
                    let mut fmt : String = String::new();
                    fmt.push_str(name);
                    fmt.push('[');
                    let baselen = fmt.len();
                    for i in 0..domsize {
                        fmt.truncate(baselen);
                        varnamegen.append_to_str(& mut fmt);
                        varnamegen.inc();
                        fmt.push(']');

                        self.task.put_var_name(varelmidx[i] as i32, fmt.as_str());
                    }
                }
                {
                    let accshape : Vec<usize> = shape[0..conedim].iter().chain(shape[conedim+1..nd].iter()).map(|v| *v).collect();
                    let accdomsize : usize = accshape.iter().product();
                    let mut accnamegen = KeyGenerator::new(accshape.as_slice());
                    let mut fmt : String = String::new();
                    fmt.push_str(name);
                    fmt.push('[');
                    let baselen = fmt.len();
                    for i in 0..accdomsize {
                        fmt.truncate(baselen);
                        accnamegen.append_to_str(& mut fmt);
                        accnamegen.inc();
                        fmt.push(']');

                        self.task.put_acc_name(accelmidx[i] as i64, fmt.as_str());
                    }
                }
            }
            None => { }
        }


        if conedim == shape.len()-1 {
            Variable{id : Some(varid as isize), shape : shape, idxs : varelmidx.iter().map(|v| (*v as isize)).collect() }
        }
        else {
            let mut permvarelmidx : Vec<isize> = vec![0; domsize];
            let d0 = shape[0..conedim].iter().product();
            let d1 = conesize;
            let d2 = shape[conedim+1..nd].iter().product();
            let mut k : usize = 0;
            for i0 in 0..d0 {
                for i1 in 0..d1 {
                    for i2 in 0..d2 {
                        let idx = i0*d1*d2+i2*d1+i1;
                        unsafe {
                            *permvarelmidx.get_unchecked_mut(k) = *varelmidx.get_unchecked(idx) as isize;
                        }
                        k += 1;
                    }
                }
            }
            Variable{id : Some(varid as isize), shape : shape, idxs : permvarelmidx }
        }
    }

    pub fn constraint(& mut self, name : Option<&str>, expr : Expression, dom : Domain) -> Constraint {
        let (dt,shape,conedim,rhs,par) = dom.unwrap();
        let domsize = shape.iter().product::<usize>();
        let conesize = shape[conedim];
        let numcone = domsize / conesize;
        let nd = shape.len();

        let mut aconelmidx : Vec<usize> = vec![0; domsize];
        let mut accelmidx : Vec<usize> = vec![0; numcone];
        let mut afeelmidx : Vec<usize> = vec![0; domsize];
        let aconid = self.acon_allocate(numcone,conesize);
        let accid = self.var_block_acc_id[aconid];
        let afeid = self.var_block_acc_id[aconid];
        self.acon_block_map.get(aconid, aconelmidx.as_mut_slice());
        self.acc_block_map.get(accid, accelmidx.as_mut_slice());
        self.afe_blocks.get(afeid, afeelmidx.as_mut_slice());

        let domidxs = self.add_domain(dt,domsize,par);

        {
            let accidxs : Vec<i64> = accelmidx.iter().map(|v| *v as i64).collect();
            let afeidxs : Vec<i64> = afeelmidx.iter().map(|v| *v as i64).collect();

            let (rowptr,subj,cof,fix,shape) = expr.unwrap();
            let nrows = rowptr.len()-1;
            let mut ptrb : Vec<i64> = vec![0; nrows];
            let mut nzrow : Vec<i32> = vec![0; nrows];
            let mut nnz = 0usize;

            for i in 0..nrows {
                ptrb[i] = nnz as i64;
                nzrow[i] = subj[rowptr[i]..rowptr[i+1]].iter().filter(|sub| sub >= 0).count();
                nnz += rownz[i] as i32;
            }

            let linsubj : Vec<i32> = subj.iter().filter(|j| j >= 0).map(|j| j as i32).collect();
            let linval  : Vec<f64> = subj.iter().zip(cof.iter()).filter(|(j,c)| j >= 0).map(|(_,c)| c).collect();

            self.task.put_acc_list(accidxs.as_slice(),domidxs.as_slice(),afeidxs.as_slice(),rhs.as_slice()).unwrap();
            if nnz < cof.len() {
                let barnnz = cof.len() - nnz;
                let barptr = vec![0usize; nrows+1];
                for i in 0..nrows {
                    barptr[i+1] = barptr[i] + subj[rowptr[i]..rowptr[i+1]].iter().filter(|j| j < 0).count();
                }

                let barelmidx = subj.iter().filter(|j| j < 0).map(|j| (-(j+1)) as usize).collect();
                let barsubj   : Vec<i32> = barelmidx.iter().map(|k| self.barvar_elm_idx[k] as i32).collect();
                let barvardim : Vec<i32> = barsubj.iter().map(|j| self.task.getdimbarvarj(j as i32).unwrap());
                let barofs    : Vec<i32> = barelmidx.iter().map(|k| self.barvar_elm_ofs[k] as i32).collect();

                let barsubk : Vec<i32> = barofs.iter().zip(barvardim.iter()).map(|(p,d)| inv_tril_idx(d,p).0).collect();
                let barsubl : Vec<i32> = barofs.iter().zip(barvardim.iter()).map(|(p,d)| inv_tril_idx(d,p).1).collect();
                let barval  : Vec<f64> = subj.iter().zip(cof.iter()).filter(|(j,c)| j < 0).map(|(_,c)| c).collect();

                let mut barafej   : Vec<i32> = Vec::new();
                let mut barafeidx : Vec<i64> = Vec::new();
                let mut matidx    : Vec<i64> = Vec::new();

                for (i,(b,e)) in barptr[0..nrows].iter().zip(barptr[1..nrows+1].zip()).enumerate() {
                    if b<e {
                        let mut p = b;
                        let mut pe = b+1;
                        let mut barj = barsubj[p];
                        let mut bardim = barvardim[p];
                        while pe < e && unsafe { barsubj.get_unchecked(pe) } == barj { ++pe; }
                        matidx.push(self.task.append_sparse_sym_mat(bardim,& barsubk[p..pe], &barsubl[p..pe], &barval[p..pe])).unwrap();
                        barafeidx.push(afeidxs[i]);
                        barafej.push(barj);
                    }
                }
                let numbarnz = barafej.len();

                self.task.put_afe_barf_entry_list(barafeidx.as_slice(),
                                                  barafej.as_slice(),
                                                  vec![1; numbarnz].as_slice(),
                                                  (0..numbarnz).collect().as_slice(),
                                                  matidx.as_slice(),
                                                  vec![1.0; numbarnz].as_slice()).unwrap();
            }
            self.task.put_afe_f_row_list(afeidxs.as_slice(),nzrow.as_slice(),ptrb.as_slice(),linsubj,linval.as_slice()).unwrap();
        }

        match name {
            Some(name) => {
                {
                    let mut varnamegen = KeyGenerator::new(shape.as_slice());
                    let mut fmt : String = String::new();
                    fmt.push_str(name);
                    fmt.push('[');
                    let baselen = fmt.len();
                    for i in 0..domsize {
                        fmt.truncate(baselen);
                        varnamegen.append_to_str(& mut fmt);
                        varnamegen.inc();
                        fmt.push(']');

                        self.task.put_var_name(varelmidx[i] as i32, fmt.as_str());
                    }
                }
                {
                    let accshape : Vec<usize> = shape[0..conedim].iter().chain(shape[conedim+1..nd].iter()).map(|v| *v).collect();
                    let accdomsize : usize = accshape.iter().product();
                    let mut accnamegen = KeyGenerator::new(accshape.as_slice());
                    let mut fmt : String = String::new();
                    fmt.push_str(name);
                    fmt.push('[');
                    let baselen = fmt.len();
                    for i in 0..accdomsize {
                        fmt.truncate(baselen);
                        accnamegen.append_to_str(& mut fmt);
                        accnamegen.inc();
                        fmt.push(']');

                        self.task.put_acc_name(accelmidx[i] as i64, fmt.as_str());
                    }
                }
            }
            None => { }
        }


        if conedim == shape.len()-1 {
            Variable{id : Some(varid as isize), shape : shape, idxs : varelmidx.iter().map(|v| (*v as isize)).collect() }
        }
        else {
            let mut permvarelmidx : Vec<isize> = vec![0; domsize];
            let d0 = shape[0..conedim].iter().product();
            let d1 = conesize;
            let d2 = shape[conedim+1..nd].iter().product();
            let mut k : usize = 0;
            for i0 in 0..d0 {
                for i1 in 0..d1 {
                    for i2 in 0..d2 {
                        let idx = i0*d1*d2+i2*d1+i1;
                        unsafe {
                            *permvarelmidx.get_unchecked_mut(k) = *varelmidx.get_unchecked(idx) as isize;
                        }
                        k += 1;
                    }
                }
            }
            Variable{id : Some(varid as isize), shape : shape, idxs : permvarelmidx }
        }

    }


    // fn named_constraint(& mut self, name : & str, expr : &Expression, dom : &Domain) -> Constraint {
    //     Constraint
    // }

    pub fn write_task(&self, filename : &str) { self.task.write_data(filename).unwrap(); }

}

pub struct Expression {
    rowptr : Vec<usize>,
    subj   : Vec<i64>,
    cof    : Vec<f64>,
    fix    : Vec<f64>,
    shape  : Vec<usize>
}

impl Expression {
    pub fn new(rowptr : &[usize], subj : &[i64], cof : &[f64], fix : &[f64]) -> Expression {
        let n = rowptr.len()-2;
        if n != fix.len() { panic!("Mismatching array lengths"); }
        if subj.len() != cof.len() { panic!("Mismatching array lengths"); }
        if ! rowptr[0..n-1].iter().zip(rowptr[1..n].iter()).all(|(a,b)| a <= b ) { panic!("Unordered rowptr array")}
        if rowptr[rowptr.len()-1] != subj.len() { panic!("Mismatching array lengths"); }

        Expression{
            shape : vec![n],
            rowptr : rowptr.to_vec(),
            subj : subj.to_vec(),
            cof : cof.to_vec(),
            fix : fix.to_vec()
        }
    }
    pub fn unwrap(self) -> (Vec<usize>,Vec<i64>,Vec<f64>,Vec<f64>,Vec<usize>) {
        let domsize = self.shape.iter().product();
        let nnz     = self.subj.len();
        if self.fix.len() != domsize { panic!("Invalid Expression");  }
        if self.rowptr.len()-1 != domsize { panic!("Invalid Expression");  }
        if self.subj.len() != self.cof.len() { panic!("Invalid Expression");  }
        if self.rowptr[self.rowptr.len()-1] != nnz { panic!("Invalid Expression");  }

        if ! rowptr[0..domsize].iter().zip(rowptr[1..domsize+1].iter()).all(|(a,b)| *a <= *b) { panic!("Invalid Expression");  }

        // reorder and compress if necessary
        let is_ordered = self.rowptr[0..domsize].iter().zip(self.rowptr[1..domsize+1].iter()).all(|(b,e)| b+1 >= e || self.subj[b..e-1].iter().zip(self.subj[b+1..e].iter()).all(|(a,b)| a < b));

        if is_ordered {
            (self.rowptr,self.subj,self.cof,self.fix,self.shape)
        }
        else {
            let mut rowptr : Vec<usize> = Vec::new(); rowptr.reserve(domsize+1); rowptr.push(0);
            let mut subj   : Vec<i64> = Vec::new(); subj.reserve(nnz);
            let mut cof    : Vec<f64> = Vec::new(); cof.reserve(nnz);
            let mut nnz : usize = 0;

            let perm : Vec<usize> = (0..nnz).collect();

            let mut nzi = 0;
            for (b,e) in self.rowptr[0..domsize].iter().zip(self.rowptr[1..domsize+1]).iter() {
                if b == e {
                }
                else if e-b == 1 {
                    rowptr.push(nnz+1);
                        subj.push( unsafe { self.subj.get_unchecked(b) });
                    ++nzi;
                }
                else {
                    (& mut perm[b..e]).sort_by_key(|i| unsafe{ *self.subj.get_unchecked(i) } );
                    subj.push(unsafe { *self.subj.get_unchecked(*perm.get_unchecked(b)) });
                    cof.push(unsafe { *self.cof.get_unchecked(*perm.get_unchecked(b)) });

                    for (pp,pi) in perm[b..e-1].iter().zip(perm[b+1..e].iter()) {
                        if pp < pi {
                            subj.push(unsafe { *self.subj.get_unchecked(*perm.get_unchecked(pi)) });
                            cof.push(unsafe { *self.cof.get_unchecked(*perm.get_unchecked(pi)) });
                            ++nzi;
                        }
                        else {
                            cof[nzi-1] += unsafe { *self.cof.get_unchecked(*perm.get_unchecked(pi)) }
                        }
                    }
                }
            }

            (rowptr,subj,cof,self.fix,self.shape)
        }
    }
}

pub struct Constraint {
    id       : Option<isize>,
    shape    : Vec<usize>,
    idxs     : Vec<isize>
}

pub struct Variable {
    id       : Option<isize>,
    shape    : Vec<usize>,
    idxs     : Vec<isize>
}
