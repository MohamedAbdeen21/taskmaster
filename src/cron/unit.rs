use std::collections::HashMap;

use anyhow::{anyhow, Error};
use chrono::{Datelike, NaiveDateTime, Timelike};
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
    pub fn range_from_str(&self, v: &str) -> Result<Vec<i32>, Error> {
        let (mut start, mut end) = match self {
            Unit::Minute => (0, 59),
            Unit::Hour => (0, 23),
            Unit::Dom => (1, 31),
            Unit::Dow => (0, 6),
            Unit::Month => (1, 12),
            _ => unreachable!(),
        };

        if v == "*" {
            return Ok((start..=end).collect_vec());
        }

        if v.contains('/') {
            let step = v.split_once('/').unwrap().1.parse()?;
            let r = v.split_once('/').unwrap().0;
            if r.contains('-') {
                let (l, r) = v.split_once('/').unwrap().0.split_once('-').unwrap();
                let (start, end) = self.validate_range(l.parse()?, r.parse()?)?;
                return Ok((start..=end).step_by(step).collect_vec());
            } else if r == "*" {
                return Ok((start..=end).step_by(step).collect_vec());
            } else {
                return Err(anyhow!(
                    "Intervals must be used with a range (* or an explicit range)"
                ));
            }
        }

        if !v.contains('/') && v.contains('-') {
            let (l, r) = v.split_once('-').unwrap();
            (start, end) = self.validate_range(l.parse()?, r.parse()?)?;
            return Ok((start..=end).collect_vec());
        }

        if let Ok(num) = v.parse() {
            (start, end) = self.validate_range(num, num)?;
            return Ok((start..=end).collect_vec());
        }

        Err(anyhow!("Value {} is not valid in field {:?}", v, self))
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

    pub fn to_hash(time: NaiveDateTime) -> HashMap<Unit, i32> {
        HashMap::from([
            (Unit::Year, Unit::Year.get(time)),
            (Unit::Month, Unit::Month.get(time)),
            (Unit::Day, Unit::Day.get(time)),
            (Unit::Hour, Unit::Hour.get(time)),
            (Unit::Minute, Unit::Minute.get(time)),
        ])
    }

    pub fn from_hash(hash: HashMap<Unit, i32>) -> NaiveDateTime {
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
    }

    pub fn parse_to_numeric(fields: [String; 5]) -> [String; 5] {
        fields
            .iter()
            .enumerate()
            .map(|(index, field)| match index {
                3 => Unit::Month.to_num(field),
                4 => Unit::Dow.to_num(field),
                _ => field.to_string(),
            })
            .collect_vec()
            .try_into()
            .unwrap()
    }
}

impl Unit {
    fn validate_range(&self, start: i32, end: i32) -> Result<(i32, i32), Error> {
        let (i, j) = match self {
            Unit::Minute => (0, 59),
            Unit::Hour => (0, 23),
            Unit::Dom => (1, 31),
            Unit::Dow => (0, 6),
            Unit::Month => (1, 12),
            _ => unreachable!(),
        };

        if start < i || end > j {
            return Err(anyhow!(
                "Value for {:?} must be between {} and {}",
                self,
                start,
                end
            ));
        }
        Ok((start, end))
    }

    fn get(&self, time: NaiveDateTime) -> i32 {
        match self {
            Unit::Year => time.year(),
            Unit::Month => time.month() as _,
            Unit::Day => time.day() as _,
            Unit::Hour => time.hour() as _,
            Unit::Minute => time.minute() as _,
            _ => unreachable!(),
        }
    }

    fn to_num(self, value: &str) -> String {
        match self {
            Unit::Dow => value
                .to_lowercase()
                .replace("sun", "0")
                .replace("mon", "1")
                .replace("tue", "2")
                .replace("wed", "3")
                .replace("thu", "4")
                .replace("fri", "5")
                .replace("sat", "6"),
            Unit::Month => value
                .to_lowercase()
                .replace("jan", "1")
                .replace("feb", "2")
                .replace("mar", "3")
                .replace("apr", "4")
                .replace("may", "5")
                .replace("jun", "6")
                .replace("jul", "7")
                .replace("aug", "8")
                .replace("sep", "9")
                .replace("oct", "10")
                .replace("nov", "11")
                .replace("dec", "12"),
            _ => unreachable!(),
        }
    }
}
