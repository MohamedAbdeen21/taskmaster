use cron::expression::Expression;
use pyo3::prelude::*;
mod cron;

/// A Python module implemented in Rust.
#[pymodule]
fn tm(_py: Python, m: &PyModule) -> PyResult<()> {
    // let submodule = PyModule::new(_py, "cron")?;
    // submodule.add_class::<Expression>()?;
    // m.add_submodule(submodule)?;

    m.add_class::<Expression>()?;
    Ok(())
}
