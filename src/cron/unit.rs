use std::collections::HashMap;

use anyhow::anyhow;
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};
use itertools::Itertools;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Unit {
    Minute,
    Dow,
    Hour,
    Dom,
    Month,

    Day,
    Year,
    None,
}

impl Unit {
    pub fn range_from_str(&self, v: &str) -> Result<Vec<i32>, anyhow::Error> {
        let mut step = 1;
        let (mut start, mut end) = match self {
            Unit::Minute => (0, 59),
            Unit::Hour => (0, 23),
            Unit::Dom => (1, 31),
            Unit::Dow => (0, 6),
            Unit::Month => (1, 12),
            _ => unreachable!(),
        };

        if v.contains('/') {
            step = v.split_once('/').unwrap().1.parse()?;
            let r = v.split_once('/').unwrap().0;
            if r.contains('-') {
                let (l, r) = v.split_once('/').unwrap().0.split_once('-').unwrap();
                let (new_start, new_end) = (l.parse::<i32>()?, r.parse::<i32>()?);
                if new_start < start || new_end > end {
                    return Err(anyhow!(
                        "Range for {:?} must be between {} and {}",
                        self,
                        start,
                        end
                    ));
                }
                (start, end) = (new_start, new_end);
            } else if r != "*" {
                return Err(anyhow!(
                    "Intervals must be used with a range (* or an explicit range)"
                ));
            }
        }

        if !v.contains('/') && v.contains('-') {
            let (l, r) = v.split_once('-').unwrap();
            let (new_start, new_end) = (l.parse::<i32>()?, r.parse::<i32>()?);
            if new_start < start || new_end > end {
                return Err(anyhow!(
                    "Range for {:?} must be between {} and {}",
                    self,
                    start,
                    end
                ));
            }
            (start, end) = (new_start, new_end);
        }

        if let Ok(num) = v.parse() {
            if num < start || num > end {
                return Err(anyhow!(
                    "Value for {:?} must be between {} and {}",
                    self,
                    start,
                    end
                ));
            }
            return Ok(vec![num]);
        }

        Ok((start..=end).step_by(step).collect_vec())
    }

    pub fn next(&self) -> Self {
        match self {
            Unit::Year => Unit::Month,
            Unit::Month => Unit::Day,
            Unit::Day => Unit::Hour,
            Unit::Hour => Unit::Minute,
            Unit::Minute => Unit::None,
            _ => unreachable!(),
        }
    }

    pub fn set(&self, time: HashMap<Unit, i32>, value: i32) -> HashMap<Unit, i32> {
        let mut time = time;
        time.insert(*self, value);
        time
    }

    fn get(&self, time: DateTime<Utc>) -> i32 {
        match self {
            Unit::Year => time.year(),
            Unit::Month => time.month() as _,
            Unit::Day => time.day() as _,
            Unit::Hour => time.hour() as _,
            Unit::Minute => time.minute() as _,
            _ => unreachable!(),
        }
    }

    pub fn to_hash(time: DateTime<Utc>) -> HashMap<Unit, i32> {
        HashMap::from([
            (Unit::Year, Unit::Year.get(time)),
            (Unit::Month, Unit::Month.get(time)),
            (Unit::Day, Unit::Day.get(time)),
            (Unit::Hour, Unit::Hour.get(time)),
            (Unit::Minute, Unit::Minute.get(time)),
        ])
    }

    pub fn from_hash(hash: HashMap<Unit, i32>) -> DateTime<Utc> {
        let year = hash[&Unit::Year];
        let month = hash[&Unit::Month];
        let day = hash[&Unit::Day];
        let hour = hash[&Unit::Hour];
        let min = hash[&Unit::Minute];
        NaiveDateTime::parse_from_str(
            &format!("{year}-{month}-{day} {hour}:{min}:00"),
            "%Y-%m-%d %H:%M:%S",
        )
        .unwrap()
        .and_utc()
    }
}
