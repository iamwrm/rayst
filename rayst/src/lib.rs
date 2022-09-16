use crossbeam::channel::{unbounded, Receiver, Sender};
use pyo3::prelude::*;
use pyo3::types::PyAny;
use std::sync::Arc;
use std::thread;

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
fn call_2_times(py_callback: &PyAny) -> PyResult<()> {
    // bring the function over to the dark side
    let callback: PyObject = py_callback.into();

    let arc_callback = Arc::new(callback);

    for _ in 0..5 {
        let c_arc_callback = arc_callback.clone();
        thread::spawn(move || {
            Python::with_gil(|py| {
                let _ = c_arc_callback.call0(py);
            });
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(6));

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
