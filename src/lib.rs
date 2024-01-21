mod cron;
mod dag;

use cron::expression::Expression;
use dag::{executor::Executor, graph::Graph, task::Task};
use pyo3::prelude::*;

#[pymodule]
fn tm(py: Python, module: &PyModule) -> PyResult<()> {
    let cron_submodule = PyModule::new(py, "cron")?;
    cron_submodule.add_class::<Expression>()?;
    module.add_submodule(cron_submodule)?;

    // allow syntax `from tm.cron import ...`
    py.run(
        "import sys; sys.modules['tm.cron'] = cron",
        None,
        Some(module.dict()),
    )?;

    module.add_class::<Graph>()?;
    module.add_class::<Task>()?;
    module.add_class::<Executor>()?;

    Ok(())
}
