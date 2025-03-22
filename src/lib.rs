use pyo3::prelude::*;
use numpy::ndarray::{ArrayD, ArrayViewD, ArrayViewMutD, Zip};
use numpy::{IntoPyArray, PyArray2, PyReadonlyArrayDyn, PyArrayMethods};

pub mod mt_ckd;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_core")]
fn sasktran2_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;

    #[pyfn(m)]
    #[pyo3(name = "mt_ckd")]
    fn mt_ckd_py<'py>(
        py: Python<'py>,
        pressure_pa: PyReadonlyArrayDyn<'py, f64>,
        temperature_k: PyReadonlyArrayDyn<'py, f64>,
        vmr_h2o: PyReadonlyArrayDyn<'py, f64>,
        vmr_co2: PyReadonlyArrayDyn<'py, f64>,
        vmr_o3: PyReadonlyArrayDyn<'py, f64>,
        path_length: f64,
    ) -> Bound<'py, PyArray2<f64>> {
        let pressure_pa = pressure_pa.as_array();
        let temperature_k = temperature_k.as_array();
        let vmr_h2o = vmr_h2o.as_array();
        let vmr_co2 = vmr_co2.as_array();
        let vmr_o3 = vmr_o3.as_array();

        let mut result = numpy::ndarray::Array2::<f64>::zeros((pressure_pa.len(), 5050));

        for i in 0..pressure_pa.len() {
            let absrb = mt_ckd::calculate_absrb(
                pressure_pa[i] / 100.0,
                temperature_k[i],
                vmr_h2o[i],
                vmr_co2[i],
                vmr_o3[i],
                path_length,
            );

            result.slice_mut(numpy::ndarray::s![i, ..]).assign(&numpy::ndarray::Array1::from_vec(absrb));
        }

        result.into_pyarray(py)
    }

    Ok(())
}
