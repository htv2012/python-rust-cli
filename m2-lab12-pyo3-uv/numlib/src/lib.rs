// lib.rs
use pyo3::prelude::*;

/// Return the factorial of N
#[pyfunction]
fn factorial(n: u64) -> PyResult<u64> {
    if n == 0 || n == 1 {
        Ok(1)
    } else {
        Ok(n * factorial(n - 1).unwrap())
    }
}

/// Provide numerical functions
#[pymodule]
fn numlib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(factorial, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn test_0() {
        assert_eq!(factorial(0).unwrap(), 1);
    }

    #[test]
    fn test_1() {
        assert_eq!(factorial(1).unwrap(), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(factorial(2).unwrap(), 2);
    }

    #[test]
    fn test_3() {
        assert_eq!(factorial(3).unwrap(), 6)
    }

    #[test]
    fn test_6() {
        assert_eq!(factorial(6).unwrap(), 720);
    }
}
