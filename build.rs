use std::env;
use std::path::PathBuf;


const MSK_MAJOR_VER : i32 = 10;
const MSK_MINOR_VER : i32 = 0;

fn get_platform_name() -> (String,String) {
    if cfg!(target_os = "windows") {
        if      cfg!(target_arch = "x86_64") {
            ("win64x86".to_string(),  format!("mosek64_{}_{}",MSK_MAJOR_VER,MSK_MINOR_VER))
        }
        else if cfg!(target_arch = "x86") {
            ("win32x86".to_string(),  format!("mosek{}_{}",MSK_MAJOR_VER,MSK_MINOR_VER))
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

fn find_mosek_installation(pfname : &String) -> String {
    let bindirvar = format!("MOSEK_BINDIR_{}{}",MSK_MAJOR_VER,MSK_MINOR_VER);

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
            bindir_b.push(format!("{}.{}",MSK_MAJOR_VER,MSK_MINOR_VER));
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

fn main() {
    // 1. if MOSEK_BINDIR is set, use that
    let (pfname, libname) = get_platform_name();
    let libdir = find_mosek_installation(&pfname);

    println!("cargo:rustc-link-search={}",libdir);
    println!("cargo:rustc-flags=-L {} -l {}",libdir,libname);

}
