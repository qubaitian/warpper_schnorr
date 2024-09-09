use pyo3::prelude::*;

/// Formats
#[pyfunction]
fn sum_as(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}


/// please
#[pyfunction]
fn other_function(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn warpper_schnorr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as, m)?)?;
    m.add_function(wrap_pyfunction!(other_function, m)?)?;
    Ok(())
}
