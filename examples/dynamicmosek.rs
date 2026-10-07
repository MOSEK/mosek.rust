//!
//!  Copyright : Copyright (c) MOSEK ApS, Denmark. All rights reserved.
//!
//!  File : dynamicmosek.rs
//!
//!  Demonstrate how to pass search paths to MOSEK in initializatin phase.
//!

use std::env;
use std::path::PathBuf;

fn main() -> Result<(),String> {
    let mut binpath = PathBuf::new();
    binpath.push("mosek");
    binpath.push(format!("{}.{}",mosek::VERSION.0,mosek::VERSION.1));
    binpath.push("tools");
    binpath.push("platform");
    match (env::consts::OS,env::consts::ARCH) {
        ("linux","x86_64") => binpath.push("linux64x86"),
        ("linux","aarch64") => binpath.push("linuxaarch64"),
        ("windows","x86_64") => binpath.push("win64x86"),
        ("macos","aarch64") => binpath.push("osxaarch64"),
        (os,arch) => return Err(format!("Platform not supported: {}/{}",os,arch))
    }
    binpath.push("bin");

    let mut paths : Vec<PathBuf> = Vec::new();
    #[cfg(target_os = "windows")]
    {
        if let Ok(v) = env::var("USERPROFILE")  {
            let mut p = PathBuf::from(&v);
            p.push(&binpath);
            paths.push(p);
        }
        if let Ok(v) = env::var("LOCALAPPDATA") {
            let mut p = PathBuf::from(&v);
            p.push(&binpath);
            paths.push(p);
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(v) = env::var("HOME") {
            let mut p = PathBuf::from(&v);
            p.push("Applications");
            p.push(&binpath);
            paths.push(p);
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(v) = env::var("HOME") {
            let mut p = PathBuf::from(&v);
            p.push(".local");
            p.push(&binpath);
            paths.push(p);

            let mut p = PathBuf::from(&v);
            p.push(&binpath);
            paths.push(p);
        }
    }


    println!("Load MOSEK with search paths:");
    for p in paths.iter() {
        println!("    {:?}",p);
    }
    mosek::initialize(Some(&paths.iter().map(|p| p.as_path()).collect::<Vec<&std::path::Path>>()))?;

    println!("Successfully loaded MOSEK {}.{} library",mosek::VERSION.0,mosek::VERSION.1);

    let mut major = 0;
    let mut minor  = 0;
    let mut revision = 0;
    mosek::get_version(&mut major, &mut minor, &mut revision)?;
    println!("   Exact MOSEK version: {}.{}.{}",major,minor,revision);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::main().unwrap();
    }
}
