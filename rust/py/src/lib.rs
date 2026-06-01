use using_dummy_variables_in_econometric_analysis_of_public_policy_core::ols_coefficients;
use numpy::{PyArray1, PyReadonlyArray1, IntoPyArray};
use pyo3::prelude::*;

#[pyfunction]
fn ols_coefficients_py<'py>(
    py: Python<'py>,
    x: PyReadonlyArray1<f64>,
    y: PyReadonlyArray1<f64>,
    n: usize,
    p: usize,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    Ok(ols_coefficients(x.as_slice()?, y.as_slice()?, n, p).into_pyarray(py))
}

#[pyfunction]
#[pyo3(signature = (x, y, n, p, iterations=2_000))]
fn bench_kernel_py(
    x: PyReadonlyArray1<f64>,
    y: PyReadonlyArray1<f64>,
    n: usize,
    p: usize,
    iterations: usize,
) -> PyResult<f64> {
    let xb = x.as_slice()?.to_vec();
    let yb = y.as_slice()?.to_vec();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = ols_coefficients(&xb, &yb, n, p);
    }
    Ok(start.elapsed().as_secs_f64())
}

#[pymodule]
fn using_dummy_variables_in_econometric_analysis_of_public_policy_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ols_coefficients_py, m)?)?;
    m.add_function(wrap_pyfunction!(bench_kernel_py, m)?)?;
    Ok(())
}
