
/**********************************************************/
#[derive(Clone)]
pub enum ConeType {
     SecondOrder,
     RotatedSecondOrder,
     Power,
     Exponential,
     DualPower,
     DualExponential,
     Psd,
}
type BoundKey = u8;
pub const BOUNDKEY_FR : BoundKey = 0;
pub const BOUNDKEY_LO : BoundKey = 1;
pub const BOUNDKEY_UP : BoundKey = 2;
pub const BOUNDKEY_FX : BoundKey = 3;

pub enum ObjectiveSense {
    Min,
    Max,
}

pub enum SolutionType {
    Default,
    Itr,
    Bas,
    Itg,
}

pub enum SolutionStatusBound {
    Optimal,
    Feasible,
    InfeasCertificate,
    IllposedCertificate,
    Any,
}

pub enum SolutionStatus {
    Optimal,
    Feasible,
    InfeasCertificate,
    IllPosedCertificate,
    Unknown,
    Undefined,
}

pub enum StatusKey {
    Basic,
    SuperBasic,
    Fixed,
    Infinite,
    AtBound,
    Unknown,
}

/**********************************************************/
/* Name generators */
pub trait IndexGenerator {
    fn next(& mut self, &mut String);
    fn skip(& mut self);
}

pub type FlatIndexer = u64;

impl IndexGenerator for FlatIndexer {
    fn next(& mut self, b : & mut String) {
        if *self == 0 {
            b.push('0')
        }
        else {
            let mut n = 1;
            while n*10 < *self { n *= 10; }
            let v = *self;
            while n > 0 {
                let c = (v / n) % 10 + ('0' as u64);
                b.push((c as u8) as char);
                n /= 10;
            }
        }
        self.skip();
    }
    fn skip(& mut self) { (*self) += 1; }
}

pub trait NameGenerator {
    fn next(& mut self, &mut String);
    fn skip(& mut self);
}

pub struct FlatNamer<IdxGen:IndexGenerator> {
    name : String,
    idx  : IdxGen
}

impl<IdxGen:IndexGenerator> FlatNamer<IdxGen> {
    fn new(name : &str, idxgen : IdxGen) -> FlatNamer<IdxGen> { return FlatNamer{ name : String::from(name), idx : idxgen} }
}

impl<IdxGen:IndexGenerator> NameGenerator for FlatNamer<IdxGen> {
    fn skip(&mut self) { self.idx.skip(); }
    fn next(&mut self, buf : & mut String) {
        buf.clear();
        buf.push_str(&self.name);
        buf.push('[');
        self.idx.next(buf);
        buf.push(']');
    }
}

pub struct NoNamer { }
impl NameGenerator for NoNamer {
    fn skip(&mut self) { }
    fn next(&mut self, buf : & mut String) { buf.clear(); }
}

pub fn no_namer() -> NoNamer { NoNamer{} }

pub fn basic_namer(name : &str) -> FlatNamer<FlatIndexer> {
    return FlatNamer{name : name.to_string(), idx : 0};
}

/**********************************************************/
/* Variable and Constraint */
pub type Variable   = Vec<i64>;
pub type Constraint = Vec<u32>;

impl Expr for Variable {
    fn len(&self) -> usize { return self.len(); }
    fn eval(&self) -> (Vec<usize>,Vec<i64>,Vec<f64>) {
        let n   = self.len();
        let ptr : Vec<usize> = (0..n+1).collect();
        let subj = self.as_slice().to_vec();
        let cof  = vec![1.0; n];

        return (ptr,subj,cof);
    }
}


/**********************************************************/
/* Domains */

fn zerosf64(n : usize) -> Vec<f64> { let mut r : Vec<f64> = Vec::with_capacity(n); r.resize(n,0.0); return r; }

pub trait Domain {
    fn size(&self) -> usize;
    fn alloc_var_block(&self,&mut Model,&str) -> Vec<i64>;
    fn alloc_con_block(&self,&mut Model,&str, ptr : &[usize], subj : &[i64], cof : &[f64]) -> Vec<u32>;
}

pub struct LinearBound {
    rhs : Vec<f64>,
    bk  : BoundKey,
}

impl LinearBound {
    pub fn size(&self) -> usize { return self.rhs.len() }
}

impl Domain for LinearBound {
    fn size(&self) -> usize { return self.rhs.len() }
    fn alloc_var_block(&self,m : &mut Model,name : &str) -> Vec<i64> {
        let idxs32 = m.alloc_vars(basic_namer(name), self.bk, self.rhs.as_slice());
        let idxs64 : Vec<i64> = idxs32.iter().map(|x| *x as i64).collect();

        return idxs64;
    }

    fn alloc_con_block(&self,m : &mut Model,
                       name : &str,
                       ptr  : &[usize],
                       subj : &[i64],
                       cof  : &[f64]) -> Vec<u32> {
        let bk =
            match self.bk {
                BOUNDKEY_FR => super::MSK_BK_FR,
                BOUNDKEY_LO => super::MSK_BK_LO,
                BOUNDKEY_UP => super::MSK_BK_UP,
                BOUNDKEY_FX => super::MSK_BK_FX,
                _ => super::MSK_BK_FR,
            };


        println!("---------- Domain::alloc");
        println!("bk = {:?}",bk);
        println!("b    = {:?}",self.rhs);
        println!("ptr  = {:?}",ptr);
        println!("subj = {:?}",subj);
        println!("cof  = {:?}",cof);
        let idxs = m.alloc_cons(basic_namer(name),
                                bk,
                                self.rhs.as_slice(),
                                ptr,
                                subj,
                                cof);
        return idxs.iter().map(|x| *x as u32).collect();
    }
}



pub struct VectorCone {
    ct   : ConeType,
    num : usize,
    par  : f64,
}

impl VectorCone {
    pub fn size(&self) -> usize { return self.num }
}

impl Domain for VectorCone {
    fn size(&self) -> usize { return self.num }
    fn alloc_var_block(&self,m : &mut Model,name : &str) -> Vec<i64> {
        let mut zs : Vec<f64> = Vec::with_capacity(self.num); zs.resize(self.num,0.0);
        let mut idxs32 = m.alloc_vars(basic_namer(name), BOUNDKEY_FR, zs.as_slice());
        m.alloc_cone(self.ct.clone(), self.par, idxs32.as_mut_slice());
        let idxs64 = idxs32.iter().map(|x| *x as i64).collect();

        return idxs64
    }

    fn alloc_con_block(&self,
                       m    : &mut Model,
                       name : &str,
                       ptr  : &[usize],
                       subj : &[i64],
                       cof  : &[f64]) -> Vec<u32> {
        let nrows = self.size();
        let mut zs : Vec<f64> = Vec::with_capacity(nrows); zs.resize(nrows,0.0);
        let idxs = m.alloc_cone_cons(basic_namer(name),
                                     self.ct.clone(),
                                     self.par,
                                     zs.as_slice(),
                                     ptr,
                                     subj,
                                     cof);
        return idxs.iter().map(|x| *x as u32).collect();
    }
}

pub fn unbounded   (num : usize)  -> LinearBound { return LinearBound{rhs : zerosf64(num), bk : BOUNDKEY_FR }; }
pub fn equal_to    (b : Vec<f64>) -> LinearBound { return LinearBound{rhs : b,             bk : BOUNDKEY_FX }; }
pub fn greater_than(b : Vec<f64>) -> LinearBound { return LinearBound{rhs : b,             bk : BOUNDKEY_LO }; }
pub fn less_than   (b : Vec<f64>) -> LinearBound { return LinearBound{rhs : b,             bk : BOUNDKEY_UP }; }

pub fn equal_to_scalar    (b : f64) -> LinearBound { return LinearBound{rhs : vec![b], bk : BOUNDKEY_FX }; }
pub fn greater_than_scalar(b : f64) -> LinearBound { return LinearBound{rhs : vec![b], bk : BOUNDKEY_LO }; }
pub fn less_than_scalar   (b : f64) -> LinearBound { return LinearBound{rhs : vec![b], bk : BOUNDKEY_UP }; }

pub fn in_second_order_cone(n : usize) -> VectorCone { return VectorCone{ct:ConeType::SecondOrder, num:n, par:0.0} }
pub fn in_rotated_second_order_cone(n : usize) -> VectorCone { return VectorCone{ct:ConeType::RotatedSecondOrder, num:n, par:0.0} }

/**********************************************************/
/* Expr */

pub trait Expr {
    fn len(&self) -> usize;
    fn eval(&self) -> (Vec<usize>,Vec<i64>,Vec<f64>);
}

impl Expr for f64 {
    fn len(&self) -> usize { return 1; }
    fn eval(&self) -> (Vec<usize>,Vec<i64>,Vec<f64>) {
        return (vec![0usize,1],
                vec![0i64],
                vec![*self])
    }
}

impl Expr for Vec<f64> {
    fn len(&self) -> usize { return self.len(); }
    fn eval(&self) -> (Vec<usize>,Vec<i64>,Vec<f64>) {
        let n = self.len();
        let ptr : Vec<usize> = (0..n+1).collect();
        return (ptr,
                vec![0; n],
                self.as_slice().to_vec());
    }
}

impl Expr for Vec<Box<Expr>> {
    fn len(&self) -> usize { return self.iter().map(|e| e.len()).sum(); }
    fn eval(&self) -> (Vec<usize>,Vec<i64>,Vec<f64>) {
        let n = self.len();
        let mut ptr : Vec<usize> = vec![0;n+1];
        let mut elmi = 0;
        let mut subj : Vec<i64> = Vec::new();
        let mut cof  : Vec<f64> = Vec::new();
        for e in self.iter() {
            let (eptr,esubj,ecof) = e.eval();
            for i in 0..eptr.len()-1 {
                ptr[elmi+1] = ptr[elmi] + (eptr[i+1]-eptr[i]);
            }
            subj.extend_from_slice(esubj.as_slice());
            cof.extend_from_slice(ecof.as_slice());
        }

        return (ptr, subj, cof);
    }
}

pub struct BaseExpr {
    ptr  : Vec<usize>,
    subj : Vec<i64>,
    cof  : Vec<f64>,
}

impl Expr for BaseExpr {
    fn len(&self) -> usize { self.ptr.len() - 1 }
    fn eval(&self) -> (Vec<usize>,Vec<i64>,Vec<f64>) {
        let mut ptr  : Vec<usize> = Vec::with_capacity(self.ptr.len());
        let mut subj : Vec<i64> = Vec::with_capacity(self.subj.len());
        let mut cof  : Vec<f64> = Vec::with_capacity(self.cof.len());
        ptr.extend_from_slice (self.ptr.as_slice());
        subj.extend_from_slice(self.subj.as_slice());
        cof.extend_from_slice (self.cof.as_slice());
        return (ptr,subj,cof);
    }
}

pub fn expr(ptr : &[usize], subj : &[i64], cof : &[f64]) -> BaseExpr {
    return BaseExpr{ ptr : ptr.to_vec(),
                     subj : subj.to_vec(),
                     cof : cof.to_vec() };
}

pub fn expr_sum(e : & impl Expr) -> BaseExpr {
    let (eptr,esubj,ecof) = e.eval();

    let n = eptr.len()-1;
    let ptr : Vec<usize> = vec![0,eptr[n]];

    return BaseExpr{ ptr  : ptr,
                     subj : esubj,
                     cof  : ecof };
}

pub fn expr_dot(c : &[f64], e : & impl Expr) -> BaseExpr {
    assert_eq!(c.len(),e.len());
    let (eptr,esubj,ecof) = e.eval();

    let n = eptr.len()-1;
    let nnz = eptr[n];
    let ptr : Vec<usize> = vec![0,nnz];
    let mut cof : Vec<f64> = vec![0.0;nnz];
    for i in 0..n {
        for j in eptr[i]..eptr[i+1] {
            cof[j] = ecof[j] * c[i];
        }
    }

    return BaseExpr{ ptr  : ptr,
                     subj : esubj,
                     cof  : cof };
}

pub fn expr_mul(mx : &(usize,usize,Vec<f64>), e : & impl Expr) -> BaseExpr {
    let (dim0_,dim1_,c) = mx;
    let dim0 : usize = *dim0_;
    let dim1 : usize = *dim1_;

    assert_eq!(dim1,e.len());
    let (eptr,esubj,ecof) = e.eval();
    let numnz = e.len();

    let ptr : Vec<usize> = (0..dim0+1).map(|i| i * numnz).collect();
    let mut subj : Vec<i64> = Vec::new();
    let mut cof  : Vec<f64> = Vec::new();

    for i in 0..dim0 {
        subj.extend_from_slice(esubj.as_slice());
        let base_idx = i * dim1;
        for j in 0..dim1 {
            for k in eptr[j]..eptr[j+1] {
                cof.push(c[base_idx + j] * ecof[k]);
            }
        }
    }

    println!("-------- expr_mul:");
    println!("ptr  = {:?}",ptr);
    println!("subj = {:?}, len = {:?}",subj,subj.len());
    println!("cof  = {:?}, len = {:?}",cof,cof.len());

    return BaseExpr{ ptr  : ptr,
                     subj : subj,
                     cof  : cof };
}

pub fn expr_stack_2(e1 : & impl Expr, e2 : & impl Expr) -> BaseExpr {
    let (eptr1,esubj1,ecof1) = e1.eval();
    let (eptr2,esubj2,ecof2) = e2.eval();

    let n1 = eptr1.len()-1;
    let n2 = eptr2.len()-1;
    let nnz1 = eptr1[n1];


    let mut ptr  : Vec<usize> = Vec::new();
    let mut subj : Vec<i64>   = Vec::new();
    let mut cof  : Vec<f64>   = Vec::new();

    ptr.extend_from_slice(eptr1.as_slice());
    for i in 0..n2 {
        ptr.push(eptr2[i+1] + nnz1);
    }
    subj.extend_from_slice(esubj1.as_slice());
    subj.extend_from_slice(esubj2.as_slice());
    cof.extend_from_slice(ecof1.as_slice());
    cof.extend_from_slice(ecof2.as_slice());

    return BaseExpr{ ptr: ptr, subj : subj, cof :cof };
}

pub fn expr_stack_3(e1 : & impl Expr, e2 : & impl Expr, e3 : & impl Expr) -> BaseExpr {
    let (eptr1,esubj1,ecof1) = e1.eval();
    let (eptr2,esubj2,ecof2) = e2.eval();
    let (eptr3,esubj3,ecof3) = e3.eval();

    let n1 = eptr1.len()-1;
    let n2 = eptr2.len()-1;
    let n3 = eptr3.len()-1;
    let nnz1 = eptr1[n1];
    let nnz2 = eptr2[n1];

    let mut ptr  : Vec<usize> = Vec::new();
    let mut subj : Vec<i64>   = Vec::new();
    let mut cof  : Vec<f64>   = Vec::new();

    ptr.extend_from_slice(eptr1.as_slice());
    for i in 0..n2 {
        ptr.push(eptr2[i+1] + nnz1);
    }
    for i in 0..n3 {
        ptr.push(eptr3[i+1] + nnz1 + nnz2);
    }
    println!("-------- expr_stack_3:");
    println!("subj1 : {:?}, cof1 : {:?}",esubj1.len(),ecof1.len());
    println!("subj2 : {:?}, cof2 : {:?}",esubj2.len(),ecof2.len());
    println!("subj3 : {:?}, cof3 : {:?}",esubj3.len(),ecof3.len());


    subj.extend_from_slice(esubj1.as_slice());
    subj.extend_from_slice(esubj2.as_slice());
    subj.extend_from_slice(esubj3.as_slice());
    cof.extend_from_slice(ecof1.as_slice());
    cof.extend_from_slice(ecof2.as_slice());
    cof.extend_from_slice(ecof3.as_slice());

    println!("ptr  = {:?}",ptr);
    println!("subj = {:?}, len = {:?}",subj,subj.len());
    println!("cof  = {:?}, len = {:?}",cof,cof.len());
    return BaseExpr{ ptr: ptr, subj : subj, cof :cof };
}


/**********************************************************/
/* Model */

pub struct Solution {
    psta : SolutionStatus,
    dsta : SolutionStatus,
    skx  : Vec<i32>,
    xx   : Vec<f64>,
    sx   : Vec<f64>,
    skc  : Vec<i32>,
    xc   : Vec<f64>,
    sc   : Vec<f64>,
}

impl Solution {
    fn new() -> Solution {
        return Solution {
            psta : SolutionStatus::Unknown,
            dsta : SolutionStatus::Unknown,
            skx  : Vec::new(),
            xx   : Vec::new(),
            sx   : Vec::new(),
            skc  : Vec::new(),
            xc   : Vec::new(),
            sc   : Vec::new(),
        };
    }
    fn resize(& mut self, numcon : usize, numvar : usize) {
        self.skx.resize(numvar,super::MSK_SK_UNK);
        self.xx.resize(numvar,0.0);
        self.sx.resize(numvar,0.0);
        self.skc.resize(numcon,super::MSK_SK_UNK);
        self.xc.resize(numcon,0.0);
        self.sc.resize(numcon,0.0);
    }
}


pub struct Model {
    env         : super::Env,
    task        : super::Task,
    slack       : Vec<i64>,
    bkx         : Vec<BoundKey>,
    numpsdatoms : i64,

    solbound    : SolutionStatusBound,
    expectsol   : SolutionType,

    // Solutions
    itr : Solution,
    bas : Solution,
    itg : Solution,
}

impl Model {
    pub fn with_name(name : &str) -> Model {
        let env = super::Env::new();
        let mut task = env.task();  //
        let slack = Vec::with_capacity(0);
        let numpsdatoms : i64 = 0;

        task.append_vars(1);
        task.put_var_name(0,"1.0");
        task.put_task_name(name);
        task.put_var_bound(0,super::MSK_BK_FX,1.0,1.0);

        task.put_task_name(name);
        if name.len() > 0 {
            let mut filename = name.to_string(); filename.push_str(".log");

            task.link_file_to_stream(super::MSK_STREAM_LOG,filename.as_str(),0);
        }

        //task.put_stream_callback(super::MSK_STREAM_LOG, |msg| print!("{}",msg));

        return Model{ env         : env,
                      task        : task,
                      slack       : slack,
                      bkx         : vec![BOUNDKEY_FX],
                      numpsdatoms : numpsdatoms,
                      solbound    : SolutionStatusBound::Optimal,
                      expectsol   : SolutionType::Default,

                      itr : Solution::new(),
                      bas : Solution::new(),
                      itg : Solution::new() };
    }

    pub fn new() -> Model {
        return Model::with_name("");
    }


    fn alloc_vars(& mut self, mut ng : impl NameGenerator, boundkey : BoundKey, b : &[f64]) -> Vec<i32> {
        let numvar = self.task.get_num_var();
        let n = b.len() as i32;
        self.task.append_vars(n as i32);

        let bk : i32 =
            match boundkey {
                BOUNDKEY_FR => super::MSK_BK_FR,
                BOUNDKEY_LO => super::MSK_BK_LO,
                BOUNDKEY_UP => super::MSK_BK_UP,
                BOUNDKEY_FX => super::MSK_BK_FX,
                _           => super::MSK_BK_FR,
            };


        let first = numvar;
        let last  = numvar+n;

        let idxs : Vec<i32>  = (first..last).collect();

        let mut nm = String::with_capacity(50);
        for i in 0..n as usize {
            ng.next(& mut nm);
            self.task.put_var_name(idxs[i],nm.as_str())
        }

        let bks : Vec<i32> = vec![bk; n as usize];
        self.task.put_var_bound_slice(numvar,numvar+n,bks.as_slice(),b,b);
        self.bkx.resize(last as usize, boundkey);
        return idxs;
    }

    fn alloc_cone(&self, ct : ConeType, cpar : f64, idxs : &[i32]) -> i32 {
        let numcone = self.task.get_num_cone();
        let nct = match ct {
            SecondOrder => super::MSK_CT_QUAD,
            RotatedSecond_Order => super::MSK_CT_RQUAD,
            Power => super::MSK_CT_PPOW,
            Exponential => super::MSK_CT_PEXP,
            DualPower => super::MSK_CT_DPOW,
            DualExponential => super::MSK_CT_DEXP,
            _ => super::MSK_CT_ZERO,
        };
        self.task.append_cone(nct,cpar,idxs);
        return numcone;
    }

    fn alloc_cons(&mut self,
                  mut ng : impl NameGenerator,
                  bk   : i32,
                  b    : &[f64],
                  ptr  : &[usize],
                  subj : &[i64],
                  cof  : &[f64] ) -> Vec<i32> {
        assert_eq!(b.len(), ptr.len()-1);

        let numcon  = self.task.get_num_con();
        let numrows = b.len();
        let n       = b.len() as i32;
        self.task.append_cons(n as i32);

        self.slack.resize(numcon as usize+numrows,0);

        let first = numcon;
        let last  = numcon+n;

        let idxs : Vec<i32>  = (first..last).collect();

        let mut nm = String::with_capacity(50);
        for i in 0..n as usize {
            ng.next(& mut nm);
            self.task.put_con_name(idxs[i],nm.as_str())
        }

        let bks : Vec<i32> = vec![bk; numrows];
        self.task.put_con_bound_slice(first,last,bks.as_slice(),b,b);


        let mut sl_ptr  : Vec<i64> = ptr.iter().map(|v| *v as i64).collect();
        let mut sl_subj : Vec<i32> = subj.iter().map(|v| *v as i32).collect();

        let (ptrb,_) = sl_ptr.split_at(numrows);
        let (_,ptre) = sl_ptr.split_at(1);

        println!("-------------");
        println!("ptr  = {:?}",ptr);
        println!("ptrb = {:?}",ptrb);
        println!("ptre = {:?}",ptre);
        println!("subj = {:?}",sl_subj);
        println!("cof  = {:?}",cof);
        println!("numvar = {:?}",self.task.get_num_var());
        self.task.put_a_row_list(idxs.as_slice(),
                                 ptrb,
                                 ptre,
                                 sl_subj.as_slice(),
                                 cof);

        return idxs;
    }

    fn alloc_cone_cons(& mut self,
                       mut ng : impl NameGenerator,
                       ct   : ConeType,
                       cpar : f64,
                       b    : &[f64],
                       ptr  : &[usize],
                       subj : &[i64],
                       cof  : &[f64] ) -> Vec<i32> {
        let nrows = ptr.len()-1;
        let nnz   = subj.len();
        let mut zs : Vec<f64> = Vec::with_capacity(nrows); zs.resize(nrows,0.0);


        println!("---------- Model::alloc_cone_cons");
        println!("b    = {:?}",b);
        println!("ptr  = {:?}",ptr);
        println!("subj = {:?}",subj);
        println!("cof  = {:?}",cof);

        //let numlinnz : u64 = subj.iter().filter(|v| v >= 0).count();

        //Following must be rewritten when we introduce PSD items
        let slacks = self.alloc_vars(no_namer(),BOUNDKEY_FR,zs.as_slice());
        let mut slacksi64 : Vec<i64> = slacks.iter().map(|i| (i+1) as i64).collect();
        self.slack.extend_from_slice(slacksi64.as_slice());
        self.alloc_cone(ct,cpar,slacks.as_slice());
        let mut sl_ptr  : Vec<i64> = Vec::with_capacity(nrows+1);
        let mut sl_subj : Vec<i32> = Vec::with_capacity(nnz+nrows);
        let mut sl_cof  : Vec<f64> = Vec::with_capacity(nnz+nrows);
        sl_ptr.push(0);

        let mut perm : Vec<usize> = (0..nnz).collect();
        for i in 0..nrows {
            // sort row
            perm[ptr[i]..ptr[i+1]].sort_by(|i0,i1| subj[*i0].cmp(&subj[*i1]));

            for j in ptr[i]..ptr[i+1] {
                sl_subj.push(subj[perm[j]] as i32);
                sl_cof.push(cof[perm[j]]);
            }
            sl_subj.push(slacks[i]);
            sl_cof.push(-1.0);
            sl_ptr.push(sl_subj.len() as i64);
        }

        let (ptrb,_) = sl_ptr.split_at(nrows);
        let (_,ptre) = sl_ptr.split_at(1);

        let first = self.task.get_num_con();
        let last = first+(nrows as i32);
        let idxs : Vec<i32> = (first..last).collect();

        self.task.append_cons(nrows as i32);

        println!("---------- put_a_row_list");
        println!("ptrb    ({:?}) = {:?}",ptrb.len(),ptrb);
        println!("ptre    ({:?}) = {:?}",ptre.len(),ptre);
        println!("sl_subj ({:?}) = {:?}",sl_subj.len(),sl_subj);
        println!("sl_cof  ({:?}) = {:?}",sl_cof.len(),sl_cof);
        println!("numvar = {:?}",self.task.get_num_var());
        self.task.put_a_row_list(idxs.as_slice(),
                                 ptrb,
                                 ptre,
                                 sl_subj.as_slice(),
                                 sl_cof.as_slice());
        return idxs;
    }

    pub fn variable<Dom:Domain>(& mut self,name : &str,dom : &Dom) -> Variable {
        let idxs = dom.alloc_var_block(self,name);
        return idxs;
    }

    pub fn constraint<Dom:Domain>(& mut self, name : &str, expr : & impl Expr, dom : &Dom) -> Constraint {
        let (ptr,subj,cof) = expr.eval();
        let idxs = dom.alloc_con_block(self,name,
                                       ptr.as_slice(),
                                       subj.as_slice(),
                                       cof.as_slice());
        return idxs;
    }

    pub fn objective(& mut self,
                     name  : &str,
                     sense : ObjectiveSense,
                     expr  : & impl Expr) {
        let (ptr,subj,cof) = expr.eval();
        //assert expr.size() <= 1
        let numvar = self.task.get_num_var();
        let mut c : Vec<f64> = Vec::with_capacity(numvar as usize); c.resize(numvar as usize,0.0);
        subj.iter().zip(cof.iter()).for_each(|(j,v)| c[*j as usize] = *v);
        let subj : Vec<i32> = (0..numvar).collect();
        self.task.put_c_list(subj.as_slice(), c.as_slice());

        match sense {
            Min => self.task.put_obj_sense(super::MSK_OBJECTIVE_SENSE_MINIMIZE),
            Max => self.task.put_obj_sense(super::MSK_OBJECTIVE_SENSE_MAXIMIZE),
        }
    }

    pub fn solve(& mut self) {
        self.task.optimize();

        let numvar = self.task.get_num_var() as usize;
        let numcon = self.task.get_num_con() as usize;

        self.itr.resize(numcon,numvar);
        self.bas.resize(numcon,numvar);
        self.itg.resize(numcon,numvar);

        if self.task.solution_def(super::MSK_SOL_ITR) {
            let solsta = self.task.get_sol_sta(super::MSK_SOL_ITR);
            let mut slx : Vec<f64> = vec![0.0; numvar];
            let mut sux : Vec<f64> = vec![0.0; numvar];
            let mut snx : Vec<f64> = vec![0.0; numvar];
            let mut skx : Vec<i32> = vec![super::MSK_SK_UNK; numvar];
            let mut slc : Vec<f64> = vec![0.0; numcon];
            let mut suc : Vec<f64> = vec![0.0; numcon];
            let mut skc : Vec<i32> = vec![super::MSK_SK_UNK; numcon];

            self.task.get_xx (super::MSK_SOL_ITR, self.itr.xx.as_mut_slice());
            self.task.get_slx(super::MSK_SOL_ITR, slx.as_mut_slice());
            self.task.get_sux(super::MSK_SOL_ITR, sux.as_mut_slice());
            self.task.get_skx(super::MSK_SOL_ITR, self.itr.skx.as_mut_slice());
            self.task.get_xc (super::MSK_SOL_ITR, self.itr.xc.as_mut_slice());
            self.task.get_slc(super::MSK_SOL_ITR, slc.as_mut_slice());
            self.task.get_suc(super::MSK_SOL_ITR, suc.as_mut_slice());
            self.task.get_skc(super::MSK_SOL_ITR, self.itr.skc.as_mut_slice());

            for i in 0..numvar {
                self.itr.sx[i] = slx[i] - sux[i] + snx[i];
            }

            for i in 0..numcon {
                if self.slack[i] == 0 {
                    self.itr.sc[i] = slc[i] - suc[i];
                } else {
                    self.itr.sc[i] = snx[(self.slack[i]-1) as usize];
                }
            }
        }
    }

    pub fn set_solution_bound(&mut self, bnd : SolutionStatusBound) { self.solbound = bnd; }
    pub fn expect_solution(&mut self, sol : SolutionType) { self.expectsol = sol; }

    pub fn write_task(& self, filename  : &str) {
        self.task.write_data(filename);
    }
}
/**********************************************************/
