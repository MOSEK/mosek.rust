use std::env;
use std::process::Command;
use std::string::String;
use std::fs;

fn getplatformname() -> String {
    let syspart = {
        let res    = Command::new("uname").output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
        let restxt = String::from_utf8(res.stdout).unwrap_or_else(|e| { panic!("non-utf-8 text: {}", e) } );

        match restxt.as_ref() {
            "Linux\n"  => "linux",
            "Darwin\n" => "osx",
            sysname  => panic!("Unsupported platform '{}'",sysname),
        }
    };

    let res = Command::new("uname").arg("-m").output().unwrap_or_else(|e| { panic!("failed to execute process: {}", e) });
    let archpart =
        match String::from_utf8(res.stdout).unwrap_or_else(|e| { panic!("non-utf-8 text: {}", e) } ).as_ref() {
            "x86_64\n" => { "64x86" }
            "i686\n"   => { "32x86" }
            archname => { panic!("Unsupported architecture {}",archname) }
        };

    return syspart.to_string()+archpart
}

fn main() {
    // 1. if MOSEK_BINDIR is set, use that

    match env::var_os("MOSEK_BINDIR") {
        Some(path) => {
            match path.into_string().ok() {
                Some(path) => {
                    println!("cargo:rustc-link-search=[\"{}\"]",path);
                    println!("cargo:rustc-flags=-L {}",path);
                    return;
                }
                None => {}
            }
        }
        None => {
            let pfname = getplatformname();
            let instbase =
                match env::var_os("MOSEK_INST_BASE") {
                    Some(path) => { path.into_string().unwrap_or_else(|_| { panic!("Invalid value of MOSEK_INST_BASE") } ) }
                    None       => { env::var_os("HOME").unwrap_or_else(|| { panic!("No MOSEK available") }).into_string().unwrap_or_else(|_| {panic!("No MOSEK available") }) }
                };

            // 2. if ~/mosek/9.0/... exists, use that
            let pfdir = instbase+"/mosek/9.0/tools/platform/"+pfname.as_ref();

            match fs::metadata(&pfdir).ok() {
                Some(md) => {
                    if md.is_dir() {
                        println!("cargo:rustc-link-search=[\"{}/bin\"]",pfdir);
                        println!("cargo:rustc-flags=-L {}/bin",pfdir);
                        return
                    }
                }
                None => {}
            }
        }
    }
    panic!("No MOSEK available");
}
