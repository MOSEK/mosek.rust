use std::env;
use std::ffi::OsStr;
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
        //else if cfg!(target_arch = "x86") {
        //    ("win32x86".to_string(),  format!("mosek{}_{}",majorver,minorver))
        //}
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
        if      cfg!(target_arch = "aarch64") {
            ("osxaarch64".to_string(),  "mosek64".to_string())
        }
        else if cfg!(target_arch = "x86_64") {
            ("osx64x86".to_string(),  "mosek64".to_string())
        }
        else {
            panic!("Unsupported architecture")
        }
    }
    else {
        panic!("Unsupported operating system")
    }
}





fn mosek_from_base_dir(p : &OsStr, pfname : &str, majorver : i32, minorver : i32) -> PathBuf {
    let mut res = PathBuf::new();
    res.push(p);
    res.push("mosek");
    res.push(format!("{}.{}",majorver,minorver));
    res.push("tools");
    res.push("platform");
    res.push(pfname);
    res.push("bin");
    res

}

/// Given platform name and version, look for a MOSEK installation in the default locations:
/// - $MOSEK_INST_BASE (all platforms)
/// - Use `which` or `where` to locate mosek binary and assume that the same directory holds the library.
/// - $HOME/mosek (on linux/osx)
/// - %HOMEDRIVE%%HOMEPATH%\mosek (on windows)
///
/// Returns `Some(path: String)` if found, otherwise `None`
fn find_mosek_installation(pfname : &String, majorver : i32, minorver : i32) -> Option<String> {
    let mosekexe =
        match pfname.as_str() {
            "win64x86" =>  "mosek.exe",
            _ => "mosek"
        };

    let mosekbin : PathBuf =
        // Traverse PATH to find mosek binary.
        // If it is found and is a symlink, follow the link.
        env::var_os("PATH")
            .and_then(|p| env::split_paths(&p).find_map(|mut pb| {
                pb.push(mosekexe);
                if pb.exists() {
                    if pb.is_symlink() {
                        std::fs::read_link(pb).ok()
                    }
                    else {
                        Some(pb)
                    }
                }
                else {
                    None
                }
            }))
            .or_else(||
                env::var_os("HOME")
                    .map(|p| { let mut pb = mosek_from_base_dir(&p, pfname, majorver, minorver); pb.push(mosekexe); pb }))
            .or_else(||
                env::var_os("HOMEDRIVE")
                    .and_then(|mut a| env::var_os("HOMEPATH").map(|b| { a.push(b); a }))
                    .map(|p| { let mut pb = mosek_from_base_dir(&p, pfname, majorver, minorver); pb.push(mosekexe); pb }))
        ?;

    if ! mosekbin.exists() { return None; }
    let mosekpath = mosekbin.parent()?;

    let res = Command::new(&mosekbin).arg("-v").output().ok()?;
    let text : String = String::from_utf8_lossy(res.stdout.as_ref()).to_string();

    let (vmajor,vminor) =
        text.strip_prefix("MOSEK version ")
            .and_then(|text| text.find('\n').map(|p| &text[0..p]) )
            .and_then(|text| {
                let mut ver = text.split('.');
                ver.next()
                    .and_then(|s| FromStr::from_str(s).ok())
                    .and_then(|v0 : i32| ver.next().and_then(|s| FromStr::from_str(s).ok().map(|v1 : i32| (v0,v1))))
                //.and_then(|(v0,v1)| if v0 == majorver && v1 == minorver { Some(true) } else { Some(false) })
            })?;
    if vmajor != majorver || vminor != minorver {
        println!("cargo:warning=Located MOSEK but version was {}.{} (expected {}.{})", vmajor,vminor,majorver,minorver);
        None
    }
    else {
        mosekpath.to_str().map(|s| s.to_string())
    }

}

// Given platform name and version, attempt to download and install the MOSEK distro.
//
// This requires the external commands
// - `curl` (all platforms)
// - `zip`/`unzip` (on windows)
// - `tar` and `bzip` (in linux/osx)
//
// Returns `Some(path: String)` on success, otherwise `None`.
fn getmosek(pfname : &String,majorver : i32, minorver : i32) -> String {
    let mut outdir = PathBuf::new();
    // OUT_DIR is defined by cargo
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


fn extract_version(text : &String) -> Option<(i32,i32)> {
    let mosekverstr = text.trim();
    match mosekverstr.find('.') {
        None => None,
        Some(p) => {
            let vmajor : i32 = FromStr::from_str(&mosekverstr[0..p]).unwrap();
            let vminor : i32 = FromStr::from_str(&mosekverstr[p+1..mosekverstr.len()]).unwrap();

            Some((vmajor,vminor))
        }
    }
}
// Read a version stored in a file. The version must have the format `[0-9]+ '.' [0-9]+`
fn readversion(filename : &str) -> (i32,i32) {
    let mut mosekverstr = String::new();
    match File::open(filename) {
        Err(_) => panic!("Failed to open version file '{}'",filename),
        Ok(mut f) => { let _ = f.read_to_string(& mut mosekverstr).unwrap(); }
    }

    match extract_version(&mosekverstr) {
        None => panic!("Invalid version file '{}'",filename),
        Some(v) => { v }
    }
}

fn main() {
    let (mskvermajor,mskverminor) = readversion("MOSEKVERSION");

    let mosekrs_force_download =
        if let Some(s) = env::var_os("MOSEKRS_FORCE_DOWNLOAD") {
            if let Some(s) = s.to_str() {
                s.eq("YES")  || s.eq("ON") || s.eq("TRUE")
            }
            else {
                false
            }
        }
        else {
            false
        };

    let (pfname, libname) = get_platform_name(mskvermajor,mskverminor);
    let libdir =
        if ! mosekrs_force_download {
            if let Some(p) = find_mosek_installation(&pfname,mskvermajor,mskverminor) { p }
            else { getmosek(&pfname, mskvermajor, mskverminor) }
        }
        else { getmosek(&pfname, mskvermajor, mskverminor) };

    println!("cargo:rustc-link-search={}",libdir);
    println!("cargo:rustc-link-lib={}",libname);

    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-arg=-Wl,-rpath=\"{}\"",libdir);
    }
    else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-rpath,\"{}\"",libdir);
    }
}
