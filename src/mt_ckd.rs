use std::fs;
use std::path::{Path, PathBuf};

/// Prepare the NetCDF file where Fortran will find it
fn ensure_mtckd_netcdf_available() -> std::io::Result<()> {
    // Path to your source NetCDF file (maybe in a config folder or bundled asset)
    let src = Path::new("/Users/djz828/dev/sasktran2_ext/vendor/MT_CKD/run_example/absco-ref_wv-mt-ckd.nc");

    // Destination is current working directory, where Fortran expects it
    let dest = Path::new("absco-ref_wv-mt-ckd.nc");

    // Avoid overwriting unnecessarily
    if !dest.exists() {
        fs::copy(&src, &dest)?;
    }

    Ok(())
}

extern "C" {
    fn set_inputs(pave: f64, tave: f64, vmr_h2o: f64, vmr_co2: f64, vmr_o3: f64, path_length: f64);
    fn run_mtckd();
    fn get_absrb(i: i32) -> f64;
}

pub fn calculate_absrb(pave: f64, tave: f64, vmr_h2o: f64, vmr_co2: f64, vmr_o3: f64, path_length: f64) -> Vec<f64> {
    unsafe {
        set_inputs(pave, tave, vmr_h2o, vmr_co2, vmr_o3, path_length);

        ensure_mtckd_netcdf_available().expect("NetCDF file missing or copy failed");

        run_mtckd();

        (0..5050).map(|i| get_absrb(i)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_absrb() {
        let absrb = calculate_absrb(1013.25, 288.15, 0.01, 400e-6, 2e-6, 1.0);
        println!("{:?}", absrb);
    }
}