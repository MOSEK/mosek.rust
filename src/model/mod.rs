pub struct Model {
    env         : super::Env,
    task        : super::Task,
    slack       : Vec<i64>,
    numpsdatoms : i64,
}

pub struct Variable {
    idxs : Vec<i64>,
}
impl Variable {
    pub fn size(&self) -> usize { return self.idxs.len() }
}

pub trait Domain {
    fn size(&self) -> usize;
    fn alloc_var_block(&self,&mut Model) -> Variable;
}

pub struct LinearBound {
    rhs : Vec<f64>,
    bk  : u8,
}
impl LinearBound {
    pub fn size(&self) -> usize { return self.rhs.len() }
}

impl Domain for LinearBound {
    fn size(&self) -> usize { return self.rhs.len() }
    fn alloc_var_block(&self,m : &mut Model) -> Variable {
        let bk =
            match self.bk & 0x03 {
                0 => super::MSK_BK_FR,
                1 => super::MSK_BK_LO,
                2 => super::MSK_BK_UP,
                _ => super::MSK_BK_FX,
            };

        let idxs32 = m.alloc_vars(self.size() as i32, bk, self.rhs.as_slice());
        let idxs64 = idxs32.iter().map(|x| *x as i64).collect();
        return Variable{ idxs:idxs64 };
    }
}



impl Model {
    pub fn new() -> Model {
        let env = super::Env::new();
        let task = env.task();  //
        let slack = Vec::with_capacity(0);
        let numpsdatoms : i64 = 0;

        task.append_vars(1);
        task.put_var_name(0,"1.0");

        return Model{ env, task, slack, numpsdatoms };
    }

    fn alloc_vars(&self, n : i32, bk : i32, b : &[f64]) -> Vec<i32> {
        let numvar = self.task.get_num_var();
        let n = b.len() as i32;
        self.task.append_vars(n as i32);

        let first = numvar;
        let last  = numvar+n;

        let idxs           = (first..last).collect();
        let bks : Vec<i32> = (0..n).map(|_| bk).collect();
        self.task.put_var_bound_slice(numvar,numvar+n,bks.as_slice(),b,b);
        return idxs;
    }

    fn alloc_cone(&self, ct : i32, cp : f64, idxs : &[i32]) -> i32 {
        let numcone = self.task.get_num_cone();
        self.task.append_cone(ct,cp,idxs);
        return numcone;
    }

    pub fn variable<Dom:Domain>(& mut self,name : &str, dom : &Dom) -> Variable {
        let v =  dom.alloc_var_block(self);

        return v;
    }

}
