use super::graph::Graph;
use anyhow::{Context, Result};
use chrono::{NaiveDateTime, Utc};
use pyo3::prelude::*;
use std::thread::sleep;

#[pyclass]
pub struct Executor {
    schedule: Vec<(NaiveDateTime, Vec<Graph>)>,
}

#[pymethods]
impl Executor {
    #[new]
    pub fn new() -> Self {
        Executor {
            schedule: Vec::new(),
        }
    }

    pub fn add(&mut self, graph: Graph) {
        let next = graph.expression.next(Utc::now().naive_utc()).unwrap();

        for (index, bucket) in self.schedule.iter_mut().enumerate() {
            if bucket.0 == next {
                bucket.1.push(graph);
                return;
            }

            if bucket.0 < next {
                self.schedule.insert(index + 1, (next, vec![graph]));
                return;
            }
        }

        self.schedule.push((next, vec![graph]));
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            let now = Utc::now().naive_utc();
            let (next, graphs) = self
                .schedule
                .pop()
                .with_context(|| "Executor doesn't have any graphs")?;
            sleep(next.signed_duration_since(now).to_std()?);
            // sleep(Duration::seconds(5).to_std()?);
            for mut graph in graphs {
                graph.start()?;
                self.add(graph);
            }
        }
    }
}
