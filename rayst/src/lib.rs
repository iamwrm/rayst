use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn multiply(a: isize, b: isize) -> PyResult<isize> {
    Ok(a * b)
}

#[pyfunction]
fn call_2_times(func: &PyAny) -> PyResult<()> {
    func.call0()?;
    func.call0()?;
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn rayst(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(multiply, m)?)?;
    m.add_function(wrap_pyfunction!(call_2_times, m)?)?;
    Ok(())
}
