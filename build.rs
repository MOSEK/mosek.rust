use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::str::FromStr;
use std::io::prelude::*;
use std::process::Command;

#[allow(unused)]
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

#[allow(unused)]
fn mosek_from_base_dir(p : &Path, pfname : &str, majorver : i32, minorver : i32) -> PathBuf {
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
/// - in `$MOSEK_INST_BASE`
/// - in `$MOSEK_BINDIR_120`, location
/// - in `PATH`, with the assumption that the library is co-located with the mosek binary
/// - in standard locations:
///   - `$HOME/.local/mosek`
///   - `$HOME/Applications/mosek` (os x only)
///   - `%USERPROFILE%/mosek` (windows only)
///   - `$HOME/mosek`
///   - `%USERPROFILE%/AppData/mosek` (windows only)
///   - `%HOMEDRIVE%/%HOMEPATH%/mosek` (windows only)
///   - `%LOCALAPPDATA%/mosek` (windows only)
///
/// Returns `Some(path: String)` if found, otherwise `None`
#[allow(unused)]
fn find_mosek_installation(pfname : &String, majorver : i32, minorver : i32) -> Option<String> {
    let bindirvar = format!("MOSEK_BINDIR_{}{}",majorver,minorver);
    let (mosekexe,libname) =
        match pfname.as_str() {
            "win64x86"     => ("mosek.exe",format!("{}mosek64_{majorver}_{minorver}{}",env::consts::DLL_PREFIX,env::consts::DLL_SUFFIX)),
            "osxaarch64"   => ("mosek",    format!("{}mosek64.{majorver}.{minorver}{}",env::consts::DLL_PREFIX,env::consts::DLL_SUFFIX)),
            _              => ("mosek",    format!("{}mosek64{}.{majorver}.{minorver}",env::consts::DLL_PREFIX,env::consts::DLL_SUFFIX))
        };

    let moseklibdir : PathBuf =
        // If MOSEK_BINDIR_XY is set, use use that
        env::var(&bindirvar).ok().map(|p| PathBuf::from(p))
            // Othrewise, of MOSEK_INST_BASE is set, use that to construct the path to the mosek library
            .or_else(|| env::var("MOSEK_INST_BASE").ok().map(|p| mosek_from_base_dir(&PathBuf::from(p), pfname, majorver, minorver)))
            // Otherwise, traverse PATH to find mosek binary. If it is found and is a symlink, follow the link, then look for the
            // mosek library in the same directory as the binary.
            .or_else(||
                env::var_os("PATH")
                    .and_then(|p| env::split_paths(&p).find_map(|path| {
                        let mut pb = PathBuf::from(&path);
                        pb.push(mosekexe);
                        let mosekexefullpath : PathBuf =
                            if !pb.exists() { return None }
                            else if ! pb.is_symlink() { pb }
                            else {
                                let pb2 = std::fs::read_link(&pb).ok()?;
                                if pb2.is_absolute() { pb2 }
                                else { pb.parent()?.join(pb2) }
                            };

                        let libdir = mosekexefullpath.parent()?;
                        let mut libfile = libdir.to_owned();
                        libfile.push(&libname);

                        if libfile.exists() {
                            Some(libdir.to_owned())
                        }
                        else {
                            None
                        }
                    })))
            // Otherwise look in standard locations
            .or_else(||
                {
                    let mut searchdirs : Vec<PathBuf> = Vec::new();
                    match env::consts::OS {
                        "windows" =>
                            // $USERPROFILE/mosek/...
                            // $LOCALAPPDATA/mosek/...
                            for key in ["USERPROFILE","LOCALAPPDATA"] {
                                if let Ok(p) = env::var(key) {
                                    searchdirs.push(PathBuf::from(p));
                                }
                            },
                        "macos" =>
                            // $HOME/Applications/mosek/...
                            // $HOME/.local/mosek/...
                            // $HOME/mosek/...
                            if let Ok(home) = env::var("HOME") {
                                let mut home = PathBuf::from(home);
                                home.push("Applications"); searchdirs.push(home.clone()); _ = home.pop();
                                home.push(".local"); searchdirs.push(home.clone()); _ = home.pop();
                                searchdirs.push(home);
                            },
                        _ =>
                            // $HOME/mosek/...
                            // $HOME/.local/mosek/...
                            if let Ok(home) = env::var("HOME") {
                                let mut home = PathBuf::from(home);
                                home.push(".local"); searchdirs.push(home.clone()); _ = home.pop();
                                searchdirs.push(home);
                            },
                    }
                    searchdirs.iter().find_map(|p| {
                        let mut moseklib = mosek_from_base_dir(p, pfname, majorver, minorver);
                        moseklib.push(&libname);
                        if moseklib.exists() { _ = moseklib.pop(); Some(moseklib) }
                        else { None }
                    })
                })?;

    println!("cargo:warning=Use MOSEK at: {}", moseklibdir.to_string_lossy());
    moseklibdir.to_str().map(|s|s.to_owned())
}

// Given platform name and version, attempt to download and install the MOSEK distro.
//
// This requires the external commands
// - `curl` (all platforms)
// - `zip`/`unzip` (on windows)
// - `tar` and `bzip` (in linux/osx)
//
// Returns `Some(path: String)` on success, otherwise `None`.
#[allow(unused)]
fn getmosek(pfname : &String,majorver : i32, minorver : i32) -> String {
    let mut outdir = PathBuf::new();
    // OUT_DIR is defined by cargo
    outdir.push(env::var_os("OUT_DIR").unwrap());
    let targetdir = outdir.as_path();
    let (archname,iszip) = match pfname.as_str() {
        "linux64x86"   => ("mosektoolslinux64x86.tar.bz2",false),
        "linuxaarch64" => ("mosektoolslinuxaarch64.tar.bz2",false),
        "osxaarch64"   => ("mosektoolsosxaarch64.tar.bz2",false),
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

#[allow(unused)]
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
#[allow(unused)]
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
    #[cfg(feature = "dynamic")]
    {
        cc::Build::new()
            .flag("-Wno-cast-function-type")
            .file("src/mosek-dynamic.c")
            .compile("mosek-dynamic");
        println!("cargo:rerun-if-changed=src/mosek-dynamic.c");
    }
    #[cfg(not(feature = "dynamic"))]
    {
        let (mskvermajor,mskverminor) = readversion("MOSEKVERSION");

        let mosekrs_force_download = env::var("MOSEKRS_FORCE_DOWNLOAD").ok()
            .map(|v| match v.as_str() { "YES"|"yes"|"ON"|"on"|"TRUE"|"true" => true, _ => false})
            .unwrap_or(false);

        let (pfname, libname) = get_platform_name(mskvermajor,mskverminor);

        let libdir : Option<String> =
            if mosekrs_force_download {
                Some(getmosek(&pfname, mskvermajor, mskverminor))
            }
        else {
            find_mosek_installation(&pfname,mskvermajor,mskverminor)
        };

        if let Some(libdir) = &libdir {
            println!("cargo:rustc-link-search={}",libdir);
        }
        println!("cargo:rustc-link-lib={}",libname);

        if let Some(libdir) = &libdir {
            if cfg!(target_os = "linux") {
                println!("cargo:rustc-link-arg=-Wl,-rpath=\"{}\"",libdir);
            }
            else if cfg!(target_os = "macos") {
                println!("cargo:rustc-link-arg=-Wl,-rpath,\"{}\"",libdir);
            }
        }
    }
}
