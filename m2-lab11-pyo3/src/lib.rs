use pyo3::prelude::*;

/// Return n!
#[pyfunction]
fn fact(n: u64) -> PyResult<u64> {
    if n == 0 || n == 1 {
        Ok(1)
    } else {
        Ok(n * fact(n - 1).unwrap())
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn factlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fact, m)?)?;
    Ok(())
}
