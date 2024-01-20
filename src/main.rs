use chrono::Utc;
use anyhow::Result;

mod cron;

fn main() -> Result<()> {
    let e = cron::expression::Expression::from_str("* * * * *").unwrap();
    let now = Utc::now().naive_utc();
    println!("{}, {}", now, e.next(now)?);
    Ok(())
}
