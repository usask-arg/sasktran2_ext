use std::env;
use std::process::Command;
use std::path::PathBuf;

fn compile_mt_ckd() {
    println!("cargo:rerun-if-changed=wrappers/MT_CKD/mt_ckd_wrap.f90");

    let target = env::var("TARGET").unwrap(); // e.g. aarch64-apple-darwin

    let profile = match target.as_str() {
        t if t.contains("apple") => "osxGNUsgl",
        t if t.contains("linux") => "linuxGNUsgl",
        t if t.contains("windows") => "winGNUsgl",
        _ => panic!("Unknown platform for MT_CKD"),
    };

    let profile_dir = match target.as_str() {
        t if t.contains("apple") => "cntnm_v4.3_OS_X_gnu_sgl.obj",
        t if t.contains("linux") => "cntnm_v4.3_linux_gnu_sgl.obj",
        t if t.contains("windows") => "cntnm_v4.3_win_gnu_sgl.obj",
        _ => panic!("Unknown platform for MT_CKD"),
    };

    let status = Command::new("make")
        .current_dir("build_scripts/MT_CKD")
        .arg("mtckd")
        .env("MTCKD_PROFILE", profile)
        .env("MTCKD_PROFILE_DIR", profile_dir)
        .status()
        .expect("Failed to run top-level Makefile");

    assert!(status.success());

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