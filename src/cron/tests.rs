use super::expression::Expression;
use anyhow::Result;
use chrono::NaiveDateTime;

fn utc_from_str(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

fn test(expression: &str, input: &str, expected: &str) -> Result<bool> {
    let expression = Expression::from_str(expression)?;
    let input = utc_from_str(input);
    let expected = utc_from_str(expected);
    Ok(expression.next(input) == expected)
}

#[test]
fn every_minute() {
    assert!(test("* * * * *", "2024-01-31 23:59:00", "2024-02-01 00:00:00").unwrap())
}
#[test]
fn every_minute_wrapping() {
    assert!(test("* * * * *", "2024-02-29 23:59:00", "2024-03-01 00:00:00").unwrap());
}

#[test]
fn minutes_list() {
    assert!(test(
        "10,20 * * * *",
        "2024-01-31 23:00:00",
        "2024-01-31 23:10:00"
    )
    .unwrap());
}

#[test]
fn inside_minutes_list() {
    assert!(test(
        "10,20 * * * *",
        "2024-01-31 23:11:00",
        "2024-01-31 23:20:00"
    )
    .unwrap());
}

#[test]
fn minutes_list_wrapping() {
    assert!(test(
        "10,20 * * * *",
        "2024-01-31 22:30:00",
        "2024-01-31 23:10:00"
    )
    .unwrap());
}

#[test]
fn inside_range_minutes() {
    assert!(test(
        "10-20 * * * *",
        "2024-01-31 22:11:00",
        "2024-01-31 22:12:00"
    )
    .unwrap());
}

#[test]
fn edges_range_minutes() {
    assert!(test(
        "10-20 * * * *",
        "2024-01-31 22:19:00",
        "2024-01-31 22:20:00"
    )
    .unwrap());
    assert!(test(
        "10-20 * * * *",
        "2024-01-31 22:20:00",
        "2024-01-31 23:10:00"
    )
    .unwrap());
}

#[test]
fn exact_minutes() {
    assert!(test("10 * * * *", "2024-01-31 22:10:00", "2024-01-31 23:10:00").unwrap());
    assert!(test("10 * * * *", "2024-01-31 22:09:00", "2024-01-31 22:10:00").unwrap());
}

#[test]
fn exact_hour() {
    assert!(test("* 10 * * *", "2024-01-31 10:01:00", "2024-01-31 10:02:00").unwrap());
    assert!(test("* 10 * * *", "2024-01-31 00:00:00", "2024-01-31 10:00:00").unwrap());
}

#[test]
fn exact_hour_wrapping() {
    assert!(test("* 10 * * *", "2024-01-31 10:59:00", "2024-02-01 10:00:00").unwrap());
}

#[test]
fn exact_month() {
    assert!(test("* * * 5 *", "2024-01-30 10:58:00", "2024-05-01 00:00:00").unwrap());
    assert!(test("* * * 5 *", "2024-05-01 00:00:00", "2024-05-01 00:01:00").unwrap());
}

#[test]
fn mix_dom_and_month() {
    assert!(test(
        "* * 15,20 2,3 *",
        "2024-01-30 10:59:00",
        "2024-02-15 00:00:00"
    )
    .unwrap());
}

#[test]
fn exact_dom_wrap_year() {
    assert!(test("* * 2 * *", "2024-12-31 23:59:00", "2025-01-02 00:00:00").unwrap());
}

#[test]
fn exact_dom_leap_year() {
    assert!(test("* * 31 * *", "2024-02-29 23:59:00", "2024-03-31 00:00:00").unwrap());
}

#[test]
fn range_dom_leap_year() {
    assert!(test(
        "* * 15-31 * *",
        "2024-02-29 23:59:00",
        "2024-03-15 00:00:00"
    )
    .unwrap());
}

#[test]
fn range_dom() {
    assert!(test(
        "* * 15-31 * *",
        "2024-03-30 23:59:00",
        "2024-03-31 00:00:00"
    )
    .unwrap());
}

#[test]
fn range_dom_wrapping() {
    assert!(test(
        "* * 15-31 * *",
        "2024-03-31 23:59:00",
        "2024-04-15 00:00:00"
    )
    .unwrap());
}

#[test]
fn mix_dow_and_dom() {
    assert!(test(
        "* * 15,20 * 3",
        "2024-01-30 10:59:00",
        "2024-01-31 00:00:00"
    )
    .unwrap());
    assert!(test(
        "* * 15,20 * 3",
        "2024-01-31 23:59:00",
        "2024-02-07 00:00:00"
    )
    .unwrap());
    assert!(test(
        "* * 15,20 * 3",
        "2024-02-07 23:59:00",
        "2024-02-14 00:00:00"
    )
    .unwrap());
    assert!(test(
        "* * 15,20 * 3",
        "2024-02-14 23:59:00",
        "2024-02-15 00:00:00"
    )
    .unwrap());
}

#[test]
fn dow_next_month() {
    assert!(test("* * * * 4", "2024-01-30 10:59:00", "2024-02-01 00:00:00").unwrap());
}

#[test]
fn dow_same_month() {
    assert!(test("* * * * 3-5", "2024-01-30 10:59:00", "2024-01-31 00:00:00").unwrap());
}

#[test]
fn dow_diff_month() {
    assert!(test("* * * 3 3", "2024-01-01 10:59:00", "2024-03-06 00:00:00").unwrap());
}

#[test]
fn dow_range() {
    assert!(test("0 6 * * 2-4", "2024-02-05 07:30:00", "2024-02-06 06:00:00").unwrap());
}

#[test]
fn dow_list() {
    assert!(test("0 6 * * 2,4", "2024-02-05 07:30:00", "2024-02-06 06:00:00").unwrap());
    assert!(test("0 6 * * 2,4", "2024-02-06 06:00:00", "2024-02-08 06:00:00").unwrap());
}

#[test]
fn feb_29_leap_year() {
    assert!(test("* * 29 2 *", "2024-03-01 10:59:00", "2028-02-29 00:00:00").unwrap());
}

#[test]
fn mix_ranges_and_lists() {
    assert!(test(
        "0 2,6-8,4 * * 0",
        "2024-01-28 06:00:00",
        "2024-01-28 07:00:00"
    )
    .unwrap());
    assert!(test(
        "0 2,6-8,4 * * 0",
        "2024-01-28 03:00:00",
        "2024-01-28 04:00:00"
    )
    .unwrap());
}

#[test]
fn list_of_ranges() {
    assert!(test(
        "15 0 1-5,10-15 * *",
        "2024-02-01 00:00:00",
        "2024-02-01 00:15:00"
    )
    .unwrap());
    assert!(test(
        "15 0 1-5,10-15 * *",
        "2024-02-09 00:00:00",
        "2024-02-10 00:15:00"
    )
    .unwrap());
    assert!(test(
        "15 0 1-5,10-15 * *",
        "2024-02-15 00:00:00",
        "2024-02-15 00:15:00"
    )
    .unwrap());
    assert!(test(
        "15 0 1-5,10-15 * *",
        "2024-02-14 00:15:00",
        "2024-02-15 00:15:00"
    )
    .unwrap());
}

#[test]
fn day_31_wrapping() {
    assert!(test("30 1 31 * *", "2024-01-31 01:31:00", "2024-03-31 01:30:00").unwrap());
}

#[test]
fn minute_interval() {
    assert!(test("*/15 * * * *", "2024-01-31 01:31:00", "2024-01-31 01:45:00").unwrap());
}

#[test]
fn minute_interval_with_range() {
    assert!(test(
        "0-30/15 * * * *",
        "2024-01-31 01:31:00",
        "2024-01-31 02:00:00"
    )
    .unwrap());
    assert!(test(
        "0-30/15 * * * *",
        "2024-01-31 02:00:00",
        "2024-01-31 02:15:00"
    )
    .unwrap());
    assert!(test(
        "0-30/15 * * * *",
        "2024-01-31 02:15:00",
        "2024-01-31 02:30:00"
    )
    .unwrap());
}

#[test]
fn month_interval() {
    assert!(test("0 0 1 */2 *", "2024-03-01 00:00:00", "2024-05-01 00:00:00").unwrap());
}

#[test]
fn month_interval_with_range() {
    assert!(test(
        "0 0 1 1-5/2 *",
        "2024-03-01 00:00:00",
        "2024-05-01 00:00:00"
    )
    .unwrap());
    assert!(test(
        "0 0 1 1-5/2 *",
        "2024-05-01 00:00:00",
        "2025-01-01 00:00:00"
    )
    .unwrap());
}

#[test]
fn dom_interval_with_range() {
    assert!(test(
        "0 0 1-10/2 * *",
        "2024-03-01 00:00:00",
        "2024-03-03 00:00:00"
    )
    .unwrap());
}

#[test]
fn mix_interval_and_ranges() {
    assert!(test(
        "1-2,*/15 * * 3 2",
        "2024-03-01 00:00:00",
        "2024-03-05 00:00:00"
    )
    .unwrap());
    assert!(test(
        "1-2,*/15 * * 3 2",
        "2024-03-05 00:00:00",
        "2024-03-05 00:01:00"
    )
    .unwrap());
    assert!(test(
        "1-2,*/15 * * 3 2",
        "2024-03-05 00:01:00",
        "2024-03-05 00:02:00"
    )
    .unwrap());
    assert!(test(
        "1-2,*/15 * * 3 2",
        "2024-03-05 00:02:00",
        "2024-03-05 00:15:00"
    )
    .unwrap());
}

#[test]
fn should_fail() {
    assert!(test("0 0 30 2 *", "", "").is_err()); // non-existent date
    assert!(test("x 0 * 2 *", "", "").is_err()); // invalid value
    assert!(test("0 0 * -1 *", "", "").is_err()); // invalid value
    assert!(test("0 0 * mon-wed *", "", "").is_err()); // wrong field
    assert!(test("0 0 * * feb", "", "").is_err()); // wrong field
    assert!(test("0 0 * jun-jan *", "", "").is_err()); // wrong order
}

#[test]
fn non_numeric_expression() {
    let expression = Expression::from_str("0 0 20 feb *").unwrap();
    let parsed = Expression::from_str("0 0 20 2 *").unwrap();
    assert_eq!(expression.fields, parsed.fields);
    let expression = Expression::from_str("0 0 20 * mon-wed").unwrap();
    let parsed = Expression::from_str("0 0 20 * 1-3").unwrap();
    assert_eq!(expression.fields, parsed.fields);
    let expression = Expression::from_str("0 0 20 * mon-wed").unwrap();
    let parsed = Expression::from_str("0 0 20 * 1-3").unwrap();
    assert_eq!(expression.fields, parsed.fields);
}
