mod cron;
mod dag;

use cron::expression::Expression;
use dag::{graph::Graph, task::Task};
use pyo3::prelude::*;
use std::include_str;

#[pymodule]
fn tm(py: Python, module: &PyModule) -> PyResult<()> {
    let cron_submodule = PyModule::new(py, "cron")?;
    cron_submodule.add_class::<Expression>()?;
    module.add_submodule(cron_submodule)?;

    let exec_impl = include_str!("./dag/executor.py");
    let executor = PyModule::from_code(py, exec_impl, "executor.py", "executor").unwrap();

    module.add_class::<Graph>()?;
    module.add_class::<Task>()?;
    module.add_submodule(executor)?;

    // expose the class directly, instead of going through the submodule
    // and allow syntax `from tm.cron import ...`
    py.run(
        "import sys; sys.modules['tm.Executor'] = executor.Executor; sys.modules['tm.cron'] = cron",
        None,
        Some(module.dict()),
    )?;

    Ok(())
}
