use std::env;
use std::process::Command;
use std::path::PathBuf;

fn compile_mt_ckd() {
    println!("cargo:rerun-if-changed=wrappers/MT_CKD/mt_ckd_wrap.f90");

    let target = env::var("TARGET").unwrap(); // e.g. "aarch64-apple-darwin" or "x86_64-apple-darwin"

    let (default_include, default_lib) = if target.contains("apple") {
        if target.contains("aarch64") {
            // Apple Silicon
            ("/opt/homebrew/include", "/opt/homebrew/lib")
        } else {
            // Intel Mac
            ("/usr/local/include", "/usr/local/lib")
        }
    } else if target.contains("linux") {
        ("/usr/include", "/usr/lib")
    } else if target.contains("windows") {
        ("/mingw64/include", "/mingw64/lib")
    } else {
        panic!("Unsupported platform: {}", target);
    };

    // Allow environment overrides
    let nc_include = env::var("NCI").unwrap_or_else(|_| default_include.to_string());
    let nc_lib = env::var("NCL").unwrap_or_else(|_| default_lib.to_string());
    let FC = env::var("FC").unwrap_or_else(|_| "gfortran".to_string());
    let PATH = env::var("PATH").unwrap_or_else(|_| "".to_string());

    let status = Command::new("make")
        .current_dir("build_scripts/MT_CKD")
        .arg("mtckd")
        .env("NCL", nc_lib)
        .env("NCI", nc_include)
        .env("FC", FC)
        .env("PATH", PATH)
        .status()
        .expect("Failed to run top-level Makefile");

    // assert!(status.success());

    println!("cargo:rustc-link-search=native=bin/");
    println!("cargo:rustc-link-lib=static=mtckd");
    println!("cargo:rustc-link-lib=gfortran");

    // Link NetCDF Fortran libraries
    println!("cargo:rustc-link-lib=netcdf");
    println!("cargo:rustc-link-lib=netcdff");

    // Add search paths if needed
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");      // for Homebrew
}


fn main() {
    println!("cargo:rerun-if-changed=dummy-file");

    compile_mt_ckd();

    // Run gfortran --print-file-name to get lib path
    let output = Command::new("gfortran")
        .arg("--print-file-name=libgfortran.a")
        .output()
        .expect("Failed to run gfortran");

    let path = String::from_utf8(output.stdout).unwrap();
    let dir = std::path::Path::new(&path).parent().unwrap();

    println!("cargo:rustc-link-search=native={}", dir.display());
    println!("cargo:rustc-link-lib=gfortran");
}