use std::env;
use std::process::Command;
use std::path::PathBuf;

fn compile_mt_ckd() {
    println!("cargo:rustc-link-search=native=bin/");
    println!("cargo:rustc-link-lib=dylib=mtckd");
    println!("cargo:rustc-link-lib=gfortran");

    // Link NetCDF Fortran libraries
    println!("cargo:rustc-link-lib=netcdf");
    println!("cargo:rustc-link-lib=netcdff");

    // Add search paths if needed
    println!("cargo:rustc-link-search=native=/opt/homebrew/lib");      // for Homebrew

    // for msys
    println!("cargo:rustc-link-search=native=C:/msys64/mingw64/lib");
    println!("cargo:rustc-link-search=native=C:/msys64/mingw64/bin");
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
