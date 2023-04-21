use std::env;
use std::path::PathBuf;
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;
//use curl::easy::Easy;
use std::process::Command;


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


fn find_mosek_installation(pfname : &String, majorver : i32, minorver : i32) -> Option<String> {
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
                                    None => return None,
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
        return None
        //panic!("MOSEK bin directory {} does not exist or is not a directory",bindir_b.as_path().to_str().unwrap());
    }

    Some(bindir_b.as_path().to_str().unwrap().to_string())
}

fn getmosek(pfname : &String,majorver : i32, minorver : i32) -> String {
    let mut outdir = PathBuf::new();
    outdir.push(env::var_os("OUT_DIR").unwrap());
    let targetdir = outdir.as_path();
    let (archname,iszip) = match pfname.as_str() {
        "linux64x86"   => ("mosektoolslinux64x86.tar.bz2",false),
        "linuxaarch64" => ("mosektoolslinuxaarch64.tar.bz2",false),
        "osx64x86"     => ("mosektoolsosx64x86.tar.bz2",false),
        "osxaarch64"   => ("mosektoolsosxaarch64.tar.bz2",false),
        "win32x86"     => ("mosektoolswin32x86.zip",true),
        "win64x86"     => ("mosektoolswin64x86.zip",true),
        _ => panic!("Invalid platform")
    };
    let mut archfile = PathBuf::new();
    archfile.push(targetdir);
    archfile.push(archname);

    if ! archfile.exists() {
        let res =  Command::new("curl")
            .arg("--silent")
            .arg(format!("https://download.mosek.com/stable/{}.{}/version",majorver,minorver).as_str())
            .output()
            .expect("Failed to get latest version");
        let verstr = match String::from_utf8_lossy(res.stdout.as_ref()) {
            std::borrow::Cow::Owned(s) => s,
            std::borrow::Cow::Borrowed(s) => s.to_string()
        };
        
        let ver = verstr.trim();

       
        Command::new("curl")
            .arg("-o").arg(archfile.as_path())
            .arg("--silent")
            .arg(format!("https://download.mosek.com/stable/{}/{}",ver,archname).as_str())
            .status()
            .expect("Failed to get distro file");

        // File written, now we have to unpack
        if iszip {
            panic!("Not implemented: Unzipping distro on Windows");
        }
        else {
            Command::new("tar")
                .arg("xjf").arg(archfile)
                .arg("-C").arg(outdir.as_path())
                .status()
                .expect("Failed to unpack distro");
        }
    }

    let mut res = PathBuf::new();
    res.push(outdir.as_path());
    res.push("mosek");
    res.push(format!("{}.{}",majorver,minorver).as_str());
    res.push("tools");
    res.push("platform");
    res.push(pfname.as_str());
    res.push("bin");
    res.as_path().to_str().unwrap().to_string()
}

fn readversion(filename : &str) -> (i32,i32) {
    let mut mosekverstr = String::new();
    match File::open(filename) {
        Err(_) => panic!("Failed to open version file '{}'",filename),
        Ok(mut f) => { let _ = f.read_to_string(& mut mosekverstr).unwrap(); }
    }
    
    let mosekverstr = mosekverstr.trim();
    match mosekverstr.find('.') {
        None => panic!("Invalid version file '{}'",filename),
        Some(p) => {
            let vmajor : i32 = FromStr::from_str(&mosekverstr[0..p]).unwrap();
            let vminor : i32 = FromStr::from_str(&mosekverstr[p+1..mosekverstr.len()]).unwrap();

            (vmajor,vminor)
        }
    }
}

fn main() {
    // 1. if MOSEK_BINDIR is set, use that
    let (mskvermajor,mskverminor) = readversion("MOSEKVERSION");

    let (pfname, libname) = get_platform_name(mskvermajor,mskverminor);
    let libdir = 
        if let Some(p) = find_mosek_installation(&pfname,mskvermajor,mskverminor) { p } 
        else { getmosek(&pfname, mskvermajor, mskverminor) };

    println!("cargo:rustc-link-search={}",libdir);
    println!("cargo:rustc-flags=-L {} -l {}",libdir,libname);
}
