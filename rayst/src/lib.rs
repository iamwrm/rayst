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

    // let mut thread_j_v = vec![];

    for _ in 0..5 {
        let c_arc_callback = arc_callback.clone();
        // let j = thread::spawn(move || {
        println!("calling callback in rust");

        let py_result = Python::with_gil(|py| -> PyResult<()> {
            println!("calling callback in rust");
            let _ = c_arc_callback.call0(py)?;
            Ok(())
        });
        py_result.unwrap();
        // });

        // thread_j_v.push(j);
    }

    // for i in thread_j_v {
    //     i.join().unwrap();
    // }

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
