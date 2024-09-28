use pyo3::prelude::*; // Import PyO3 prelude to use its macros

#[pyfunction]
fn add(a: i32, b: i32) -> PyResult<i32> {
    Ok(a + b)
}

#[pymodule]
fn rx_orm(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?; // Register the add function with the module
    Ok(())
}