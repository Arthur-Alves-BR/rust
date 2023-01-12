use std::time::Duration;
use time::OffsetDateTime;
use time::PrimitiveDateTime as DateTime;

fn main() {
    let now = OffsetDateTime::now_utc();
    let mut datetime = DateTime::new(now.date(), now.time());
    
    let interval = Duration::from_secs(1_000_000_000);
    
    datetime = datetime + interval;

    println!("{}", datetime);
}
