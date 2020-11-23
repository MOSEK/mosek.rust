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
struct IndexManager {
    n          : usize, // capacity
    link       : Vec<isize>,
    first_free : isize, // index of the first element in the free block, -1 if empty
    last_free  : isize, // index of the last element in the free block, -1 if empty
    nfree      : usize  // current number of free elements
}

impl IndexManager {
    pub fn new(n : usize) -> IndexManager {
        let mut link = vec![0; 2*n];
        unsafe {
            for i in 0..n-1 {
                *link.get_unchecked_mut(2*i) = (i+1) as isize;
                *link.get_unchecked_mut(2*(i+1)+1) = i as isize;
            }
            *link.get_unchecked_mut(2*(n-1)) = -1 as isize;
            *link.get_unchecked_mut(1)       = -1 as isize;
        }

        IndexManager{ n : n, link:link, first_free : 0, last_free  : 0, nfree : n }
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
            self.first_free = self.link[tail*2];
            self.link[tail*2] = -1; // next if block[last] = -1
            self.link[self.first_free as usize + 1] = -1; // previous if free[first] = -1;
        }

        self.link[head*2+1] = -2;

        head
    }

    // Free the block pointed to by head, placing it at the end of the
    // current free list
    pub fn release(& mut self, head : usize) {
        if head >= self.n || self.link[head*2+1] != -2 {
            panic!("Index is not head of an allocated block");
        }

        let mut blocklast = head;
        let mut n : usize = 1;
        unsafe {
            while *self.link.get_unchecked(blocklast*2) >= 0 {
                blocklast = *self.link.get_unchecked(blocklast*2) as usize;
                n += 1;
            }
        }
        self.link[head*2] -= 1;


        self.link[head*2+1] = self.last_free; // prev[head] = last_free

        if self.first_free < 0 {
            self.first_free = head as isize;
        } else {
            self.link[(self.last_free*2) as usize] = head as isize; // next[last_free] = head
        }

        self.last_free = blocklast as isize;
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

    pub fn get(&self, p : usize, idxs : & mut Vec<usize>) {
        let mut i = p as isize;
        while i >= 0 {
            idxs.push(i as usize);
            i = self.link[(i as usize)*2];
        }
    }
}

struct BlockManager {
    idxs : IndexManager,
    blks : IndexManager,
    heads : Vec<usize>
}

impl BlockManager {
    pub fn new() -> BlockManager {
        BlockManager{
            idxs : IndexManager::new(128),
            blks : IndexManager::new(128),
            heads : vec!(128)
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


    pub fn get(&self, blockid : usize,  idxs : & mut Vec<usize> ) { self.idxs.get(self.heads[blockid],idxs) }
    pub fn blockmaxindex(&self, blockid : usize) -> usize { self.idxs.blockmaxindex(self.heads[blockid]) }
}




// struct CodeRowManager {
//     ptr  : Vec<usize>,
//     size : Vec<usize>,

//     nused       : usize,
//     firstunused : usize,
//     code   : Vec<u8>,
//     cconst : Vec<f64>
// }

// impl CodeRowManager {
//     pub fn new() {
//         CodeRowManager{
//             ptr    : Vec::new(),
//             size   : Vec::new(),
//             nused  : 0,
//             firstunused : 0,
//             code   : Vec::new(),
//             cconst : Vec::new()
//         }
//     }

//     pub fn resize(& mut self, n : usize) {
//         if n > self.ptr.len() {
//             self.ptr.resize(n,0);
//             self.size.resize(n,0);
//         }
//         else if n < self.ptr().len() {
//             for i in n .. self.ptr.len() {
//                 self.nused -= self.size[i];
//             }
//             self.ptr.resize(n,0);
//             self.size.resize(n,0);
//         }
//     }

//     pub fn getrow(& self, i : usize) -> Option<(& SliceIndex<[u8]>::Output,& SliceIndex<[f64]>::Output)> {
//         if i >= self.size.len() {
//             return None;
//         }
//         else {
//             let p  = self.ptr[i];
//             let sz = self.size[i];
//             return Some(self.code[p..p+sz],self.cconst[p..p+sz]);
//         }
//     }

//     fn compress(& mut self) {
//         let mut code   : Vec<u8> = vec![0; self.nused];
//         let mut cconst : Vec<f64> = vec![0; self.nused];
//         let mut ptr    : Vec<usize> = vec![0; self.ptr.len()];

//         let mut p : usize = 0;
//         unsafe {
//             let ptr = self.ptr.as_ptr();
//             let selfcode   = self.code.as_ptr();
//             let selfcconst = self.cconst.as_ptr();
//             let codep      = code.as_mut_ptr();
//             let cconstp    = cconst.as_mut_ptr();

//             for i in 0..self.ptr.len() {
//                 ptr.get_mut_unchecked(i) = p;
//                 let sz = self.size.get_unchecked(i);
//                 let srcp = self.ptr.get_unchecked(i);
//                 for j in 0..sz { codep[p+j]   = selfcode[srcp+j]; }
//                 for j in 0..sz { cconstp[p+j] = selfcconst[srcp+j]; }

//                 p += sz;
//             }
//         }

//         self.ptr.clone_from_slice(ptr.as_slice());
//         self.code[..self.nused].clone_from_slice(code.as_slice());
//         self.cconst[..self.nused].clone_from_slice(cconst.as_slice());
//         self.firstunused = self.nused;
//     }

//     pub fn replacerow(& mut self, i : usize, n : usize) -> Option<(& mut SliceIndex<[u8]>::Output,& mut SliceIndex<[f64]>::Output)> {
//         if i >= self.size.len() {
//             return None;
//         }
//         else {
//             self.nused -= self.size[i];
//             self.size[i] = 0;

//             if (self.nused+n)*2 < self.code.len() {
//                 self.compress();
//             }

//             if self.code.len() - self.firstunused < n {
//                 let newn = self.code.len() - self.firstunused + n;
//                 self.code.resize(newn,0);
//                 self.cconst.resize(newn,0);
//             }

//             self.nused -= self.size[i];
//             self.nused += n;

//             let p  = self.firstunused;
//             self.firstunused += n;
//             self.ptr[i] = p;
//             self.size[i] = n;
//             return Some(self.code[p..p+n],self.cconst[p..p+n]);
//         }
//     }
// }

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
    var_block_acc_id : Vec<MaybeUsize>,
    // maps varelmidx -> ACC idx
    var_elm_acc_idx : Vec<MaybeUsize>,
    // maps varelmidx -> ACC offset
    var_elm_acc_ofs : Vec<MaybeUsize>,

    barvar_block_map : BlockManager,
    barvar_ptr : Vec<usize>,
    barvar_elm_idx : Vec<usize>,
    barvar_elm_ofs : Vec<usize>
}

impl Model {
    fn new() -> Model {
        let env : super::Env = super::Env::new().unwrap();
        let task : super::Task = env.task().unwrap();

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
            // maps varid -> accid. For k=var_block_acc_id[i], if k > 0 then
            // (k-1) is the id of the associated ACC block.
            var_block_acc_id : Vec::new(),
            // maps varelmidx -> ACC idx
            var_elm_acc_idx : Vec::new(),
            // maps varelmidx -> ACC offset
            var_elm_acc_ofs : Vec::new(),

            barvar_block_map : BlockManager::new(),
            barvar_ptr : vec![0],
            barvar_elm_idx : Vec::new(),
            barvar_elm_ofs : Vec::new()
        }
    }

    fn linearvar_allocate(& mut self, n : usize) -> usize {
        let varid = self.var_block_map.allocate(n);
        let minnumvar = self.var_block_map.blockmaxindex(varid)+1;
        let numvar = self.task.get_num_var().unwrap() as usize;
        if numvar < minnumvar {
            self.task.append_vars((minnumvar-numvar) as i32).unwrap();
        }

        if varid+1 > self.var_block_acc_id.len() { self.var_block_acc_id.resize(varid+1,MaybeUsize::none()); }

        if self.var_elm_acc_idx.len() < minnumvar {
            self.var_elm_acc_idx.resize(minnumvar,MaybeUsize::none());
            self.var_elm_acc_ofs.resize(minnumvar,MaybeUsize::none());
        }
        varid
    }

    fn conicvar_allocate(& mut self, numcone : usize, conesize : usize) -> usize {
        let n = numcone*conesize;
        let varid = self.linearvar_allocate(n);
        let accid = self.acc_allocate(numcone);
        self.var_block_acc_id[varid] = MaybeUsize::some(accid);
        let mut varelmidxs : Vec<usize> = Vec::new();
        let mut accelmidxs : Vec<usize> = Vec::new();
        self.var_block_map.get(varid, & mut varelmidxs);
        self.acc_block_map.get(accid, & mut accelmidxs);
        unsafe {
            //let (accidxptr,accofsptr) = (self.var_block_acc_idx.as_mut_ptr(),self.var_block_acc_ofs.as_mut_ptr());
            //let varelmidxsptr = varelmidxs.as_ptr();
            //let accelmidxsptr = accelmidxs.as_ptr();
            let mut k : usize = 0;
            for i in 0..numcone {
                for j in 0..conesize {
                    let varelmidx : usize = *varelmidxs.get_unchecked(k);
                    *self.var_elm_acc_idx.get_unchecked_mut(varelmidx) = MaybeUsize::some(*accelmidxs.get_unchecked(i));
                    *self.var_elm_acc_ofs.get_unchecked_mut(varelmidx) = MaybeUsize::some(j);
                    k += 1;
                }
            }
        }
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
            //let ptrp = self.barvar_ptr.as_mut_ptr();
            let mut p = firstelm;
            for i in first..first+n {
                *self.barvar_ptr.get_unchecked_mut(i) = p;
                p += conedim*(conedim+1)/2;
            }
            //let (elmidxptr,elmofsptr) = (self.barvar_elm_idx.as_mut_ptr(),self.barvar_elm_ofs.as_mut_ptr());
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
        0
    }

    fn acon_allocate(& mut self, numcone : usize, conesize : usize) -> usize {
        0
    }

    fn acc_allocate(& mut self, numcone : usize) -> usize {
        0
    }

}

pub struct Constraint {
    id       : isize,
    original : bool,

    shape    : Vec<usize>,
    idxs     : Vec<usize>
}

pub struct Variable {
    id       : isize,
    original : bool,

    shape    : Vec<usize>,
    idxs     : Vec<i64>
}
