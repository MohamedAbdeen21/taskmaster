use chrono::Utc;

mod cron;

fn main() {
    let e = cron::expression::Expression::from_str("* * * * *").unwrap();
    let now = Utc::now().naive_utc();
    println!("{}, {}", now, e.next(now));
}
