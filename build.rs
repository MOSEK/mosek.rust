use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;

fn get_platform_name(majorver : i32,minorver : i32) -> (String,String) {
    if cfg!(target_os = "windows") {
        if      cfg!(target_arch = "x86_64") {
            ("win64x86".to_string(),  format!("mosek64_{}_{}",majorver,minorver))
        }
        else if cfg!(target_arch = "x86") {
            ("win32x86".to_string(),  format!("mosek{}_{}",majorver,minorver))
        }
        else {
            panic!("Unsupported architecture")
        }
    }
    else if cfg!(target_os = "linux") {
        if      cfg!(target_arch = "x86_64") {
            ("linux64x86".to_string(),"mosek64".to_string())
        }
        else if cfg!(target_arch = "aarch64") {
            ("linuxaarch64".to_string(),"mosek64".to_string())
        }
        else {
            panic!("Unsupported architecture")
        }
    }
    else if cfg!(target_os = "macos") {
        if      cfg!(target_arch = "x86_64") {
            ("osx64x86".to_string(),  "mosek64".to_string())
        }
        else if cfg!(target_arch = "aarch64") {
            ("osxaarch64".to_string(),  "mosek64".to_string())
        }
        else {
            panic!("Unsupported architecture")
        }
    }
    else {
        panic!("Unsupported operating system")
    }
}

fn find_mosek_installation(pfname : &String, majorver : i32, minorver : i32) -> String {
    let bindirvar = format!("MOSEK_BINDIR_{}{}",majorver,minorver);

    let mut bindir_b = PathBuf::new();
    match env::var_os(bindirvar.as_str()) {
        Some(p) => bindir_b.push(p),
        None    => {
            let inst_base =
                match env::var_os("MOSEK_INST_BASE") {
                    Some(p) => p,
                    None =>
                        match env::var_os("HOME") {
                            Some(p) => p,
                            None =>
                                match env::var_os("HOMEPATH") {
                                    Some(p) => {
                                        let mut r = env::var_os("HOMEDRIVE").unwrap();
                                        r.push(p);
                                        r },
                                    None => panic!("No MOSEK installation found"),
                                }
                        }
                };
            bindir_b.push(inst_base);
            bindir_b.push("mosek");
            bindir_b.push(format!("{}.{}",majorver,minorver));
            bindir_b.push("tools");
            bindir_b.push("platform");
            bindir_b.push(pfname);
            bindir_b.push("bin");
            },
        }

    if ! bindir_b.as_path().is_dir() {
        panic!("MOSEK bin directory {} does not exist or is not a directory",bindir_b.as_path().to_str().unwrap());
    }

    bindir_b.as_path().to_str().unwrap().to_string()
}

fn readversion(filename : &str) -> (i32,i32) {
    let mut mosekverstr = String::new();
    match File::open(filename) {
        Err(_) => panic!("Failed to open version file '{}'",filename),
        Ok(mut f) => { let _ = f.read_to_string(& mut mosekverstr).unwrap(); }
    }
    match mosekverstr.find('.') {
        None => panic!("Invalid version file '{}'",filename),
        Some(p) => {
            let vmajor : i32 = FromStr::from_str(&mosekverstr[0..p]).unwrap();
            let vminor : i32 = FromStr::from_str(&mosekverstr[p+1..mosekverstr.len()-1]).unwrap();

            (vmajor,vminor)
        }
    }
}

fn main() {
    // 1. if MOSEK_BINDIR is set, use that
    let (mskvermajor,mskverminor) = readversion("MOSEKVERSION");

    let (pfname, libname) = get_platform_name(mskvermajor,mskverminor);
    let libdir = find_mosek_installation(&pfname,mskvermajor,mskverminor);

    println!("cargo:rustc-link-search={}",libdir);
    println!("cargo:rustc-flags=-L {} -l {}",libdir,libname);

}
