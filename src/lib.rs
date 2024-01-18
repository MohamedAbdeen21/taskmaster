mod cron;
mod dag;

use cron::expression::Expression;
use dag::{graph::Graph, task::Task};
use pyo3::prelude::*;

#[pymodule]
fn tm(_py: Python, m: &PyModule) -> PyResult<()> {
    // let submodule = PyModule::new(_py, "cron")?;
    // submodule.add_class::<Expression>()?;
    // m.add_submodule(submodule)?;

    m.add_class::<Expression>()?;
    m.add_class::<Graph>()?;
    m.add_class::<Task>()?;
    Ok(())
}
