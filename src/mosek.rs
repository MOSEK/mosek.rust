extern crate libc;
use std::ffi::CString;
use libc::c_char;

#[link(name = "mosek64")]
extern
{
    fn MSK_getversion(major : * mut i32, 
                      minor : * mut i32,
                      build : * mut i32,
                      rev   : * mut i32) -> i32;

    fn MSK_makeenv(env : * mut * const u8, dbgfile : * const c_char) -> i32;
    fn MSK_deleteenv(env : * mut * const u8) -> i32;
    fn MSK_maketask(env : * const u8, maxnumcon : i32, maxnumvar : i32, task : * mut * const u8) -> i32;
    fn MSK_deletetask(task : * mut * const u8) -> i32;

    fn MSK_readdata(task : * const u8,    filename : * const c_char) -> i32;
    fn MSK_writedata(task : * const u8,   filename : * const c_char) -> i32;
    fn MSK_putarow(task  : * const u8,    i : libc::int32_t, nzi : libc::int32_t, subi : * const libc::int32_t, vali : * const f64) -> i32;
    fn MSK_getarow(task  : * const u8,    i : libc::int32_t, nzi : * mut libc::int32_t, subi : * mut libc::int32_t, vali : * mut f64) -> i32;
    fn MSK_getarownumnz(task  : * const u8,    i : libc::int32_t, nzi : * mut libc::int32_t) -> i32;
    fn MSK_optimizetrm(task : * const u8, trmcode  : * mut i32) -> i32;
}

struct Env
{
    env : * const u8,
}
struct Task
{
    task : * const u8,
}

impl Env
{
    fn new() -> Env
    {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, std::ptr::null()) };
        if res != 0 { panic!("Failed: MSK_getversion"); }

        return Env { env : env };
    }

    fn newMemDebug(dbgfile : &str) -> Env
    {
        let mut env : * const u8 = std::ptr::null();
        let res = unsafe { MSK_makeenv(& mut env, CString::new(dbgfile).unwrap().as_ptr()) };
        if res != 0 { panic!("Failed: MSK_makeenv"); }

        return Env { env : env };
    }

    fn newtask(&self) -> Task
    {
        let mut task : * const u8 = std::ptr::null();
        if 0 != unsafe { MSK_maketask(self.env, 0,0, & mut task) }
        { 
            panic!("Failed: MSK_maketask"); 
        }

        return Task { task : task };
    }
}




impl Task
{
    fn readdata(&self, filename : &str)
    {
        if 0 != (unsafe { MSK_readdata(self.task, CString::new(filename).unwrap().as_ptr()) })
        {
            panic!("Failed: MSK_readdata");
        }
    }

    fn writedata(&self, filename : &str)
    {
        if 0 != unsafe { MSK_writedata(self.task, CString::new(filename).unwrap().as_ptr()) }
        {
            panic!("Failed: MSK_writedata");
        }
    }


    fn getarownumnz(&self, i : i32) -> i32
    {
        let mut nzi : i32 = 0;
        if 0 != unsafe { MSK_getarownumnz(self.task, i, & mut nzi) }
        {
            panic!("Failed: MSK_getarownumnz");
        }
        return nzi;
    }

    fn getarow(&self, i : i32, subi : & mut Vec<i32>, vali : & mut Vec<f64>) -> i32
    {
        let mut nzi = self.getarownumnz(i);
        let mincap = nzi as usize;

        if (subi.capacity() < mincap) { let subilen = subi.len(); subi.reserve(mincap - subilen); }
        if (vali.capacity() < mincap) { let valilen = vali.len(); vali.reserve(mincap - valilen); }

        if 0 != (unsafe { MSK_getarow(self.task, i, & mut nzi, subi.as_mut_ptr(), vali.as_mut_ptr()) })
        {
            panic!("Failed: MSK_getarow");
        }

        unsafe
        {
            subi.set_len(mincap);
            vali.set_len(mincap);
        }

        return nzi;
    }

    fn putarow(&self, i:i32, subi : & Vec<i32>, vali : & Vec<f64>)
    {
        if (subi.len() != vali.len()) { panic!("Failed: MSK_getarow. Mismatching array lengths"); }

        if 0 != (unsafe { MSK_putarow(self.task, i, subi.len() as i32, subi.as_ptr(), vali.as_ptr()) })
        {
            panic!("Failed: MSK_getarow");
        }
    }

    fn optimize(&self) -> i32
    {
        let mut res = 0;
        if 0 != unsafe { MSK_optimizetrm(self.task, & mut res) }
        {
            panic!("Failed: MSK_optimizetrm");
        }

        return res;
    }
}


impl Drop for Env
{
    fn drop( & mut self)
    {
        let mut env = self.env;
        unsafe { MSK_deleteenv(& mut env); };
    }
}

impl Drop for Task
{
    fn drop( & mut self)
    {
        let mut task = self.task;
        unsafe { MSK_deletetask(& mut task) };
    }
}


fn getversion() -> (i32,i32,i32,i32)
{
    let mut major    : i32 = 0;
    let mut minor    : i32 = 0;
    let mut build    : i32 = 0;
    let mut revision : i32 = 0;

    let res = unsafe { MSK_getversion(& mut major,& mut minor,& mut build,& mut revision) };
    if res != 0 { panic!("Failed: MSK_getversion"); }

    return (major,minor,build,revision);
}



fn main() {
    let (major,minor,build,revision) = getversion();
    println!("MOSEK version {}.{}.{}.{}", major,minor,build,revision);

    let env = Env::new();
    let task = env.newtask();
    task.readdata("/home/ulfw/mosekprj/git/dev/tests/opf/25fv47.opf");
    task.optimize();
    task.writedata("25fv47.task");
}
