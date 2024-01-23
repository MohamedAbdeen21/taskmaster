use chrono::{Datelike, NaiveDate};
use days_in_month::days_in_month;
use itertools::Itertools;
use std::collections::HashMap;

use super::unit::Unit;

pub fn get_next(values: &[i32], unit: i32) -> (i32, bool) {
    if let Some(v) = values.iter().find(|&&v| v > unit) {
        return (*v, false);
    }

    return (*values.first().unwrap(), true);
}

pub fn next_month(time: HashMap<Unit, i32>) -> HashMap<Unit, i32> {
    let mut time = time;
    if time[&Unit::Month] == 12 {
        time.entry(Unit::Year).and_modify(|v| *v += 1);
        time = Unit::Month.set(time, 0);
    }
    time.entry(Unit::Month).and_modify(|v| *v += 1);
    time = Unit::Day.set(time, 1);
    time = Unit::Hour.set(time, 0);
    // Minutes are either reset or incremented to next value (0)
    time = Unit::Minute.set(time, -1);
    time
}

pub fn adjust_days_to_month(
    schedule: &HashMap<Unit, Vec<i32>>,
    year: i32,
    month: i32,
) -> HashMap<Unit, Vec<i32>> {
    let mut schedule = schedule.clone();

    // Month is not in schedule and therefore has no days
    if !schedule[&Unit::Month].contains(&(month as _)) {
        return schedule;
    }

    let max_days = days_in_month(year, month as _) as i32;

    let first_dow = NaiveDate::from_ymd_opt(year, month as _, 1)
        .unwrap()
        .weekday()
        .num_days_from_monday() as i32;

    let first_appearances = schedule
        .get(&Unit::Dow)
        .unwrap_or(&vec![])
        .iter()
        .map(|day| (day - first_dow) % 7)
        .collect_vec();

    let dow_to_dom = first_appearances
        .iter()
        .flat_map(|day| (0..5).map(|i| day + 7 * i).collect_vec())
        .collect_vec();

    let days = schedule
        .get(&Unit::Dom)
        .unwrap_or(&vec![])
        .iter()
        .chain(dow_to_dom.iter())
        .cloned()
        .filter(|&day| day > 0 && day <= max_days)
        .sorted()
        .dedup()
        .collect_vec();

    schedule.insert(Unit::Day, days);
    schedule
}
