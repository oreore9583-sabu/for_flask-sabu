use pyo3::prelude::*;

#[pyfunction]
fn calc(expr: String) -> PyResult<String> {
    Ok(format!("Rust接続OK: {}", expr))
}

#[pymodule]
fn my_calc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc, m)?)?;
    Ok(())
}
