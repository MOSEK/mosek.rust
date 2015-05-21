extern crate libc;

#[link(name = "mosek64")]
extern
{
    fn MSK_getversion(major : * mut i32, 
                      minor : * mut i32,
                      build : * mut i32,
                      rev   : * mut i32) -> i32;
}

fn getversion() -> (i32,i32,i32,i32)
{
    let mut major    : i32 = 0;
    let mut minor    : i32 = 0;
    let mut build    : i32 = 0;
    let mut revision : i32 = 0;

    let res = unsafe { MSK_getversion(& mut major,& mut minor,& mut build,& mut revision) };
    if res != 0
    {
        panic!("Failed: MSK_getversion");
    }

    return (major,minor,build,revision);
}

fn main() {
    let (major,minor,build,revision) = getversion();
    println!("MOSEK version {}.{}.{}.{}", major,minor,build,revision);
}
