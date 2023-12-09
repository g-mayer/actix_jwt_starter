use chrono::{DateTime, Utc};
use chrono_tz::Tz;

// If you are going to use this function remove line 6, otherwise delete module.
#[allow(unused)]
pub fn convert_utc_to_local(utc_time: DateTime<Utc>, timezone_str: &str) -> DateTime<Tz> {
    let timezone: Tz = timezone_str.parse().expect("Invalid timezone");
    utc_time.with_timezone(&timezone)
}
