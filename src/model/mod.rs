pub struct Model {
    env         : super::Env,
    task        : super::Task,
    slack       : Vec<i64>,
    numpsdatoms : i64,
}



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

        return Model{ env:env, task:task, slack:slack, numpsdatoms:numpsdatoms };
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

    pub fn variable<Dom:Domain>(& mut self, dom : &Dom) -> Variable {
        let v =  dom.alloc_var_block(self);
        return v;
    }

}
