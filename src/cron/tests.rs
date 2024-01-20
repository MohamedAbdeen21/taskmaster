use chrono::NaiveDateTime;

use super::expression::Expression;

fn utc_from_str(s: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S").unwrap()
}

#[test]
fn every_minute() {
    let expression = Expression::from_str("* * * * *").unwrap();
    let input = utc_from_str("2024-01-31 23:59:00");
    let expected = utc_from_str("2024-02-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn every_minute_wrapping() {
    let expression = Expression::from_str("* * * * *").unwrap();
    let input = utc_from_str("2024-02-29 23:59:00");
    let expected = utc_from_str("2024-03-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn minutes_list() {
    let expression = Expression::from_str("10,20 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 23:00:00");
    let expected = utc_from_str("2024-01-31 23:10:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn inside_minutes_list() {
    let expression = Expression::from_str("10,20 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 23:11:00");
    let expected = utc_from_str("2024-01-31 23:20:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn minutes_list_wrapping() {
    let expression = Expression::from_str("10,20 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 22:30:00");
    let expected = utc_from_str("2024-01-31 23:10:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn inside_range_minutes() {
    let expression = Expression::from_str("10-20 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 22:11:00");
    let expected = utc_from_str("2024-01-31 22:12:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn edges_range_minutes() {
    let expression = Expression::from_str("10-20 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 22:19:00");
    let expected = utc_from_str("2024-01-31 22:20:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-31 22:20:00");
    let expected = utc_from_str("2024-01-31 23:10:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn exact_minutes() {
    let expression = Expression::from_str("10 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 22:10:00");
    let expected = utc_from_str("2024-01-31 23:10:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-31 22:09:00");
    let expected = utc_from_str("2024-01-31 22:10:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn exact_hour() {
    let expression = Expression::from_str("* 10 * * *").unwrap();
    let input = utc_from_str("2024-01-31 10:01:00");
    let expected = utc_from_str("2024-01-31 10:02:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-31 00:00:00");
    let expected = utc_from_str("2024-01-31 10:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn exact_hour_wrapping() {
    let expression = Expression::from_str("* 10 * * *").unwrap();
    let input = utc_from_str("2024-01-31 10:59:00");
    let expected = utc_from_str("2024-02-01 10:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn exact_month() {
    let expression = Expression::from_str("* * * 5 *").unwrap();
    let input = utc_from_str("2024-01-30 10:58:00");
    let expected = utc_from_str("2024-05-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-05-01 00:00:00");
    let expected = utc_from_str("2024-05-01 00:01:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn mix_dom_and_month() {
    let expression = Expression::from_str("* * 15,20 2,3 *").unwrap();
    let input = utc_from_str("2024-01-30 10:59:00");
    let expected = utc_from_str("2024-02-15 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn exact_dom_wrap_year() {
    let expression = Expression::from_str("* * 2 * *").unwrap();
    let input = utc_from_str("2024-12-31 23:59:00");
    let expected = utc_from_str("2025-01-02 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn exact_dom_leap_year() {
    let expression = Expression::from_str("* * 31 * *").unwrap();
    let input = utc_from_str("2024-02-29 23:59:00");
    let expected = utc_from_str("2024-03-31 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn range_dom_leap_year() {
    let expression = Expression::from_str("* * 15-31 * *").unwrap();
    let input = utc_from_str("2024-02-29 23:59:00");
    let expected = utc_from_str("2024-03-15 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn range_dom() {
    let expression = Expression::from_str("* * 15-31 * *").unwrap();
    let input = utc_from_str("2024-03-30 23:59:00");
    let expected = utc_from_str("2024-03-31 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn range_dom_wrapping() {
    let expression = Expression::from_str("* * 15-31 * *").unwrap();
    let input = utc_from_str("2024-03-31 23:59:00");
    let expected = utc_from_str("2024-04-15 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn mix_dow_and_dom() {
    let expression = Expression::from_str("* * 15,20 * 3").unwrap();
    let input = utc_from_str("2024-01-30 10:59:00");
    let expected = utc_from_str("2024-01-31 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-31 23:59:00");
    let expected = utc_from_str("2024-02-07 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-02-07 23:59:00");
    let expected = utc_from_str("2024-02-14 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-02-14 23:59:00");
    let expected = utc_from_str("2024-02-15 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn dow_next_month() {
    let expression = Expression::from_str("* * * * 4").unwrap();
    let input = utc_from_str("2024-01-30 10:59:00");
    let expected = utc_from_str("2024-02-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn dow_same_month() {
    let expression = Expression::from_str("* * * * 3-5").unwrap();
    let input = utc_from_str("2024-01-30 10:59:00");
    let expected = utc_from_str("2024-01-31 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn dow_diff_month() {
    let expression = Expression::from_str("* * * 3 3").unwrap();
    let input = utc_from_str("2024-01-01 10:59:00");
    let expected = utc_from_str("2024-03-06 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn dow_range() {
    let expression = Expression::from_str("0 6 * * 2-4").unwrap();
    let input = utc_from_str("2024-02-05 07:30:00");
    let expected = utc_from_str("2024-02-06 06:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn dow_list() {
    let expression = Expression::from_str("0 6 * * 2,4").unwrap();
    let input = utc_from_str("2024-02-05 07:30:00");
    let expected = utc_from_str("2024-02-06 06:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-02-06 06:00:00");
    let expected = utc_from_str("2024-02-08 06:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn feb_29_leap_year() {
    let expression = Expression::from_str("* * 29 2 *").unwrap();
    let input = utc_from_str("2024-03-01 10:59:00");
    let expected = utc_from_str("2028-02-29 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn mix_ranges_and_lists() {
    let expression = Expression::from_str("0 2,6-8,4 * * 0").unwrap();
    let input = utc_from_str("2024-01-28 06:00:00");
    let expected = utc_from_str("2024-01-28 07:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-28 03:00:00");
    let expected = utc_from_str("2024-01-28 04:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn list_of_ranges() {
    let expression = Expression::from_str("15 0 1-5,10-15 * *").unwrap();
    let input = utc_from_str("2024-02-01 00:00:00");
    let expected = utc_from_str("2024-02-01 00:15:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-02-09 00:00:00");
    let expected = utc_from_str("2024-02-10 00:15:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-02-15 00:00:00");
    let expected = utc_from_str("2024-02-15 00:15:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-02-14 00:15:00");
    let expected = utc_from_str("2024-02-15 00:15:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn day_31_wrapping() {
    let expression = Expression::from_str("30 1 31 * *").unwrap();
    let input = utc_from_str("2024-01-31 01:31:00");
    let expected = utc_from_str("2024-03-31 01:30:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn minute_interval() {
    let expression = Expression::from_str("*/15 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 01:31:00");
    let expected = utc_from_str("2024-01-31 01:45:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn minute_interval_with_range() {
    let expression = Expression::from_str("0-30/15 * * * *").unwrap();
    let input = utc_from_str("2024-01-31 01:31:00");
    let expected = utc_from_str("2024-01-31 02:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-31 02:00:00");
    let expected = utc_from_str("2024-01-31 02:15:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-01-31 02:15:00");
    let expected = utc_from_str("2024-01-31 02:30:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn month_interval() {
    let expression = Expression::from_str("0 0 1 */2 *").unwrap();
    let input = utc_from_str("2024-03-01 00:00:00");
    let expected = utc_from_str("2024-05-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn month_interval_with_range() {
    let expression = Expression::from_str("0 0 1 1-5/2 *").unwrap();
    let input = utc_from_str("2024-03-01 00:00:00");
    let expected = utc_from_str("2024-05-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-05-01 00:00:00");
    let expected = utc_from_str("2025-01-01 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn dom_interval_with_range() {
    let expression = Expression::from_str("0 0 1-10/2 * *").unwrap();
    let input = utc_from_str("2024-03-01 00:00:00");
    let expected = utc_from_str("2024-03-03 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn mix_interval_and_ranges() {
    let expression = Expression::from_str("1-2,*/15 * * 3 2").unwrap();
    let input = utc_from_str("2024-03-01 00:00:00");
    let expected = utc_from_str("2024-03-05 00:00:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-03-05 00:00:00");
    let expected = utc_from_str("2024-03-05 00:01:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-03-05 00:01:00");
    let expected = utc_from_str("2024-03-05 00:02:00");
    assert_eq!(expression.next(input).unwrap(), expected);
    let input = utc_from_str("2024-03-05 00:02:00");
    let expected = utc_from_str("2024-03-05 00:15:00");
    assert_eq!(expression.next(input).unwrap(), expected);
}

#[test]
fn should_fail() {
    assert!(Expression::from_str("0 0 30 2 *").is_err()); // non-existant date
    assert!(Expression::from_str("x 0 * 2 *").is_err()); // invalid value
    assert!(Expression::from_str("0 0 * -1 *").is_err()); // invalid value
    assert!(Expression::from_str("0 0 * mon-wed *").is_err()); // wrong field
    assert!(Expression::from_str("0 0 * * feb").is_err()); // wrong field
    assert!(Expression::from_str("0 0 * jun-jan *").is_err()) // wrong order
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
