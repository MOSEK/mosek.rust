
struct IndexGenerator<'a> {
    shape : & 'a [usize],
    m     : usize,
    i     : usize,
    value : usize
}

impl<'a> IndexGenerator<'a> {
    fn new(shape : &'a[usize], value : usize) -> IndexGenerator<'a> {
        IndexGenerator{shape:shape, m : shape.iter().product(), i : 0, value : value }
    }
}

impl<'a> Iterator for IndexGenerator<'a> {
    type Item = usize;
    fn next(& mut self) -> Option<usize> {
        if self.i < self.shape.len() {
            self.m /= self.shape[self.i];
            self.i += 1;
            Some((self.value / self.m) % self.shape[self.i-1])
        }
        else {
            None
        }
    }
}


// shaped_namer
pub struct ShapedNameGenerator<'a> {
    shape    : & 'a [usize],
    baselen  : usize,
    buf      : String,
}

pub fn shaped<'a>(name : &str, shape : &'a [usize]) -> ShapedNameGenerator<'a> {
    ShapedNameGenerator{ shape : shape, baselen : name.len(),buf : name.to_string() } //
}

impl<'a> ShapedNameGenerator<'a> {
    pub fn get(& mut self, idx : usize) -> & str {
        self.buf.truncate(self.baselen);
        self.buf.push('[');

        if self.shape.len() > 0 {
            let mut idxgen = IndexGenerator::new(self.shape,idx);
            itoa(idxgen.next().unwrap(),& mut self.buf);
            for idxval in idxgen { self.buf.push(','); itoa(idxval,& mut self.buf); }
        }
        self.buf.push(']');

        self.buf.as_str()
    }
}

// sparse_namer

pub struct SparseNameGenerator<'a> {
    shape : &'a [usize],
    sp    : &'a [usize],
    baselen : usize,
    buf   : String,
}

impl<'a> SparseNameGenerator<'a> {
    pub fn get<'b>(&'b mut self, idx : usize) -> & 'b str {
        self.buf.truncate(self.baselen);
        self.buf.push('[');
        if self.shape.len() > 0 {
            let mut idxgen = IndexGenerator::new(self.shape,self.sp[idx]);
            itoa(idxgen.next().unwrap(),& mut self.buf);
            for idxval in idxgen { self.buf.push(','); itoa(idxval,& mut self.buf); }
        }
        self.buf.push(']');
        self.buf.as_str()
    }
}

pub fn sparse<'a>(name : & str, shape : &'a [usize], sp : &'a [usize]) -> SparseNameGenerator<'a> {
    SparseNameGenerator{ shape : shape, sp : sp, baselen : name.len(), buf : name.to_string() }
}


fn itoa(i : usize, b : & mut String) {
    if i == 0 {
        b.push('0')
    }
    else {
        let mut n = 1;
        while n*10 <= i { n *= 10; }
        let v = i;
        while n > 0 {
            let c = (v / n) % 10 + ('0' as usize);
            b.push((c as u8) as char);
            n /= 10;
        }
    }
}
