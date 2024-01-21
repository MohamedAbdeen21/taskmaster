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
        let insertion_index = self
            .schedule
            .iter()
            .position(|v| v.0 <= next)
            .unwrap_or(self.schedule.len());

        let new_bucket = (next, vec![graph.clone()]);

        if insertion_index == self.schedule.len() {
            self.schedule.push(new_bucket);
            return;
        }

        let bucket = self.schedule.get_mut(insertion_index).unwrap();
        if bucket.0 == next {
            bucket.1.push(graph);
        } else {
            self.schedule.insert(insertion_index, new_bucket)
        }
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            let now = Utc::now().naive_utc();
            let (next, graphs) = self
                .schedule
                .pop()
                .with_context(|| "Executor doesn't have any graphs")?;
            // let now = Utc::now().naive_utc();
            // let next = self.expression.next(now)?;
            sleep(next.signed_duration_since(now).to_std()?);
            // sleep(Duration::seconds(5).to_std()?);
            for mut graph in graphs {
                graph.start()?;
                self.add(graph);
            }
        }
    }
}
