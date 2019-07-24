pub struct Model {
    env         : super::Env,
    task        : super::Task,
    slack       : Vec<i64>,
    numpsdatoms : i64,
}

/**********************************************************/
type ConeType = u8;
pub const CONE_TYPE_SECOND_ORDER         : ConeType = 0;
pub const CONE_TYPE_ROTATED_SECOND_ORDER : ConeType = 1;
pub const CONE_TYPE_POWER                : ConeType = 2;
pub const CONE_TYPE_EXPONENTIAL          : ConeType = 3;
pub const CONE_TYPE_DUAL_POWER           : ConeType = 4;
pub const CONE_TYPE_DUAL_EXPONENTIAL     : ConeType = 5;
//pub const CONE_TYPE_PSD                  : ConeType = 6;

type BoundKey = u8;
pub const BOUNDKEY_FR : BoundKey = 0;
pub const BOUNDKEY_LO : BoundKey = 1;
pub const BOUNDKEY_UP : BoundKey = 2;
pub const BOUNDKEY_FX : BoundKey = 3;

type ObjectiveSense = u8;
pub const OBJECTIVE_SENSE_MIN : ObjectiveSense = 0;
pub const OBJECTIVE_SENSE_MAX : ObjectiveSense = 1;

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

pub type Variable   = Vec<u64>;
pub type Constraint = Vec<u32>;

/**********************************************************/
/* Domains */

pub trait Domain {
    fn size(&self) -> usize;
    fn alloc_var_block(&self,&mut Model,&str) -> Vec<u64>;
    fn alloc_con_block(&self,&mut Model,&str, ptr : &[usize], subj : &[u64], cof : &[f64]) -> Vec<u32>;
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
    fn alloc_var_block(&self,m : &mut Model,name : &str) -> Vec<u64> {
        let bk =
            match self.bk {
                BOUNDKEY_FR => super::MSK_BK_FR,
                BOUNDKEY_LO => super::MSK_BK_LO,
                BOUNDKEY_UP => super::MSK_BK_UP,
                BOUNDKEY_FX => super::MSK_BK_FX,
                _ => super::MSK_BK_FR,
            };

        let idxs32 = m.alloc_vars(basic_namer(name), bk, self.rhs.as_slice());
        let idxs64 : Vec<u64> = idxs32.iter().map(|x| *x as u64).collect();

        return idxs64;
    }

    fn alloc_con_block(&self,m : &mut Model,
                       name : &str,
                       ptr  : &[usize],
                       subj : &[u64],
                       cof  : &[f64]) -> Vec<u32> {
        let bk =
            match self.bk {
                BOUNDKEY_FR => super::MSK_BK_FR,
                BOUNDKEY_LO => super::MSK_BK_LO,
                BOUNDKEY_UP => super::MSK_BK_UP,
                BOUNDKEY_FX => super::MSK_BK_FX,
                _ => super::MSK_BK_FR,
            };

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
    fn alloc_var_block(&self,m : &mut Model,name : &str) -> Vec<u64> {
        let mut zs : Vec<f64> = Vec::with_capacity(self.num); zs.resize(self.num,0.0);
        let mut idxs32 = m.alloc_vars(basic_namer(name), super::MSK_BK_FR, zs.as_slice());
        m.alloc_cone(self.ct, self.par, idxs32.as_mut_slice());
        let idxs64 = idxs32.iter().map(|x| *x as u64).collect();

        return idxs64
    }

    fn alloc_con_block(&self,
                       m    : &mut Model,
                       name : &str,
                       ptr  : &[usize],
                       subj : &[u64],
                       cof  : &[f64]) -> Vec<u32> {
        let nrows = self.size();
        let mut zs : Vec<f64> = Vec::with_capacity(nrows); zs.resize(nrows,0.0);
        let idxs = m.alloc_cone_cons(basic_namer(name),
                                     self.ct,
                                     self.par,
                                     zs.as_slice(),
                                     ptr,
                                     subj,
                                     cof);
        return idxs.iter().map(|x| *x as u32).collect();
    }
}

/**********************************************************/
/* Expr */

pub trait Expr {
    fn size(&self) -> usize;
    fn eval(&self) -> (Vec<usize>,Vec<u64>,Vec<f64>);
}

pub struct BaseExpr {
    ptr  : Vec<usize>,
    subj : Vec<u64>,
    cof  : Vec<f64>,
}

impl Expr for BaseExpr {
    fn size(&self) -> usize { self.ptr.len() - 1 }
    fn eval(&self) -> (Vec<usize>,Vec<u64>,Vec<f64>) {
        let mut ptr  : Vec<usize> = Vec::with_capacity(self.ptr.len());
        let mut subj : Vec<u64> = Vec::with_capacity(self.subj.len());
        let mut cof  : Vec<f64> = Vec::with_capacity(self.cof.len());
        ptr.extend_from_slice (self.ptr.as_slice());
        subj.extend_from_slice(self.subj.as_slice());
        cof.extend_from_slice (self.cof.as_slice());
        return (ptr,subj,cof);
    }
}

/**********************************************************/
/* Model */

impl Model {
    pub fn new() -> Model {
        let env = super::Env::new();
        let task = env.task();  //
        let slack = Vec::with_capacity(0);
        let numpsdatoms : i64 = 0;

        task.append_vars(1);
        task.put_var_name(0,"1.0");

        return Model{ env:env, task:task, slack:slack, numpsdatoms:numpsdatoms };
    }

    fn alloc_vars(&self, mut ng : impl NameGenerator, bk : i32, b : &[f64]) -> Vec<i32> {
        let numvar = self.task.get_num_var();
        let n = b.len() as i32;
        self.task.append_vars(n as i32);

        let first = numvar;
        let last  = numvar+n;

        let idxs : Vec<i32>  = (first..last).collect();

        let mut nm = String::with_capacity(50);
        for i in 0..n as usize {
            ng.next(& mut nm);
            self.task.put_var_name(idxs[i],nm.as_str())
        }

        let bks : Vec<i32> = (0..n).map(|_| bk).collect();
        self.task.put_var_bound_slice(numvar,numvar+n,bks.as_slice(),b,b);
        return idxs;
    }

    fn alloc_cone(&self, ct : ConeType, cpar : f64, idxs : &[i32]) -> i32 {
        let numcone = self.task.get_num_cone();
        let nct = match ct {
            CONE_TYPE_SECOND_ORDER => super::MSK_CT_QUAD,
            CONE_TYPE_ROTATED_SECOND_ORDER => super::MSK_CT_RQUAD,
            CONE_TYPE_POWER => super::MSK_CT_PPOW,
            CONE_TYPE_EXPONENTIAL => super::MSK_CT_PEXP,
            CONE_TYPE_DUAL_POWER => super::MSK_CT_DPOW,
            CONE_TYPE_DUAL_EXPONENTIAL => super::MSK_CT_DEXP,
            _ => super::MSK_CT_ZERO,
        };
        self.task.append_cone(nct,cpar,idxs);
        return numcone;
    }

    fn alloc_cons(&self,
                  mut ng : impl NameGenerator,
                  bk   : i32,
                  b    : &[f64],
                  ptr  : &[usize],
                  subj : &[u64],
                  cof  : &[f64] ) -> Vec<i32> {
        // assert b.len() == ptr.len()-1
        let numcon = self.task.get_num_con();
        let n      = b.len() as i32;
        self.task.append_cons(n as i32);

        let first = numcon;
        let last  = numcon+n;

        let idxs : Vec<i32>  = (first..last).collect();

        let mut nm = String::with_capacity(50);
        for i in 0..n as usize {
            ng.next(& mut nm);
            self.task.put_con_name(idxs[i],nm.as_str())
        }

        let bks : Vec<i32> = (0..n).map(|_| bk).collect();
        self.task.put_con_bound_slice(first,last,bks.as_slice(),b,b);

        return idxs;
    }

    fn alloc_cone_cons(&self,
                       mut ng : impl NameGenerator,
                       ct   : ConeType,
                       cpar : f64,
                       b    : &[f64],
                       ptr  : &[usize],
                       subj : &[u64],
                       cof  : &[f64] ) -> Vec<i32> {
        let nrows = ptr.len()-1;
        let nnz   = subj.len();
        let mut zs : Vec<f64> = Vec::with_capacity(nrows); zs.resize(nrows,0.0);

        //let numlinnz : u64 = 0;subj.iter().filter(|v| v >= 0).count();

        //Following must be rewritten when we introduce PSD items
        let slacks = self.alloc_vars(no_namer(),super::MSK_BK_FR,zs.as_slice());
        self.alloc_cone(ct,cpar,slacks.as_slice());
        let mut sl_ptr  : Vec<i64> = Vec::with_capacity(nrows+1);
        let mut sl_subj : Vec<i32> = Vec::with_capacity(nnz+nrows);
        let mut sl_cof  : Vec<f64> = Vec::with_capacity(nnz+nrows);
        sl_ptr.push(0);
        for i in 0..nnz {
            for j in ptr[i]..ptr[i+1] {
                sl_subj.push(subj[j] as i32);
                sl_cof.push(cof[j]);
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

    pub fn constraint<Dom:Domain>(& mut self, name : &str, expr : impl Expr, dom : &Dom) -> Constraint {
        let (ptr,subj,cof) = expr.eval();
        let idxs = dom.alloc_con_block(self,name,
                                       ptr.as_slice(),
                                       subj.as_slice(),
                                       cof.as_slice());
        return idxs;
    }

    pub fn objective(& mut self, name : &str, sense : ObjectiveSense, expr : impl Expr) {
        let (ptr,subj,cof) = expr.eval();
        //assert expr.size() <= 1
        let numvar = self.task.get_num_var();
        let mut c : Vec<f64> = Vec::with_capacity(numvar as usize); c.resize(numvar as usize,0.0);
        subj.iter().zip(cof.iter()).for_each(|(j,v)| c[*j as usize] = *v);
        let subj : Vec<i32> = (0..numvar).collect();
        self.task.put_c_list(subj.as_slice(), c.as_slice());
    }
}
