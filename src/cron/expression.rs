use anyhow::{anyhow, Error, Result};
use chrono::{Datelike, NaiveDateTime, Utc};
use itertools::Itertools;
use pyo3::prelude::*;
use std::collections::HashMap;

use super::helpers::{adjust_days_to_month, get_next, next_month};
use super::unit::Unit;

const DOM: usize = 2;
const DOW: usize = 4;

#[pyclass]
#[derive(Debug)]
pub struct Expression {
    pub fields: [String; 5],
}

#[pymethods]
impl Expression {
    #[new]
    pub fn from_str(expression: &str) -> Result<Self, Error> {
        let fields: [String; 5] = expression
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect_vec()
            .try_into()
            .map_err(|_| anyhow!("Expression {} should have 5 fields", expression))?;

        let fields = Unit::parse_to_numeric(fields);

        let e = Expression { fields };
        _ = e.next(Utc::now().naive_utc())?; // ensure expression is valid
        Ok(e)
    }

    pub fn next(&self, now: NaiveDateTime) -> Result<NaiveDateTime> {
        let schedule = self.create_schedule(now.year(), now.month() as _)?;
        let (next, _) =
            Self::calculate_next_time(Unit::Year, false, &schedule, Unit::to_hash(now))?;
        Ok(Unit::from_hash(next))
    }
}

impl Expression {
    fn calculate_next_time(
        unit: Unit,
        reset: bool,
        schedule: &HashMap<Unit, Vec<i32>>,
        time: HashMap<Unit, i32>,
    ) -> Result<(HashMap<Unit, i32>, bool)> {
        let mut time = time;
        let mut schedule = schedule.clone();

        // No days schedules for this month, keep incrementing months till we find some days
        // the iterative approach helps with leap years
        // If we go five years without finding a date, then the expression contains an invalid date
        // like Feb-30 for example
        let start_year = time[&Unit::Year];
        while !schedule.contains_key(&Unit::Day) || schedule[&Unit::Day].is_empty() {
            if time[&Unit::Year] == start_year + 5 {
                return Err(anyhow!("Invalid Expression"));
            }
            time = next_month(time);
            schedule = adjust_days_to_month(&schedule, time[&Unit::Year], time[&Unit::Month])
        }

        if unit == Unit::None {
            return Ok((time, false));
        }

        // A higher field changed its value, need to reset all lower fields
        if reset {
            return Self::calculate_next_time(
                unit.next(),
                reset,
                &schedule,
                unit.set(time, *schedule[&unit].first().unwrap()),
            );
        }

        // If minutes are not reset, then go to next value
        if unit == Unit::Minute {
            let (next_value, of) = get_next(&schedule[&unit], time[&unit]);
            return Ok((unit.set(time, next_value), of));
        }

        // unit is in schedule, increment only if lower fields overflow
        if schedule[&unit].contains(&time[&unit]) {
            let (mut time, of) = Self::calculate_next_time(unit.next(), false, &schedule, time)?;
            if !of {
                return Ok((time, of));
            }

            let (next_value, of) = get_next(&schedule[&unit], time[&unit]);
            time = unit.set(time, next_value);

            // The month has changed, re-calculate the dom and dow
            // and reset lower fields
            if unit == Unit::Month {
                schedule = adjust_days_to_month(&schedule, time[&Unit::Year], time[&Unit::Month]);
                (time, _) = Self::calculate_next_time(unit.next(), true, &schedule, time)?;
            }

            return Ok((time, of));
        }

        // unit is not in schedule, increment and reset lower fields
        if !schedule[&unit].contains(&time[&unit]) {
            let (next_value, of) = get_next(&schedule[&unit], time[&unit]);
            time = unit.set(time, next_value);

            // The month has changed, re-calculate the dom and dow
            if unit == Unit::Month {
                schedule = adjust_days_to_month(&schedule, time[&Unit::Year], time[&Unit::Month]);
            }

            // reset lower fields
            (time, _) = Self::calculate_next_time(unit.next(), true, &schedule, time)?;
            return Ok((time, of));
        }

        Ok((time, false))
    }

    fn create_schedule(&self, year: i32, month: i32) -> Result<HashMap<Unit, Vec<i32>>, Error> {
        let mut schedule = HashMap::with_capacity(7); // 7 variants of Unit
        let mut ignore = Vec::new();

        schedule.insert(Unit::Year, (year..year + 20).collect_vec());

        // handle interaction with ranges
        if self.fields[DOM] == "*" && self.fields[DOW] != "*" {
            ignore.push(DOM);
        }
        if self.fields[DOW] == "*" && self.fields[DOM] != "*" {
            ignore.push(DOW);
        }

        for (index, field) in self.fields.iter().enumerate() {
            if ignore.contains(&index) {
                continue;
            }

            let unit = match index {
                0 => Unit::Minute,
                1 => Unit::Hour,
                2 => Unit::Dom,
                3 => Unit::Month,
                4 => Unit::Dow,
                _ => unreachable!(),
            };

            // replacing .flat_map mainly to properly bubble the error
            let mut range = Vec::new();
            for part in field.split(',') {
                let r = unit.range_from_str(part)?;
                range.extend(r);
            }

            schedule.insert(unit, range.into_iter().sorted().collect::<Vec<_>>());
        }

        Ok(adjust_days_to_month(&schedule, year, month))
    }
}
