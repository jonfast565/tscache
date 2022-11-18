use chrono::{NaiveDateTime, Utc, DateTime};

pub fn convert_i64_to_utc_datetime(timestamp: i64) -> DateTime<Utc> {
    let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0);
    let datetime: DateTime<Utc> = DateTime::from_utc(naive.unwrap(), Utc);
    datetime
}