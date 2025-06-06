use chrono::NaiveDateTime;
use time::OffsetDateTime;

pub fn convert_naive_to_offset(value: NaiveDateTime) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(value.and_utc().timestamp()).unwrap()
}

pub fn convert_offset_to_naive(value: OffsetDateTime) -> NaiveDateTime {
    let secs = value.unix_timestamp();
    let nanos = value.nanosecond();
    // Convert to chrono's NaiveDateTime
    #[allow(deprecated)]
    NaiveDateTime::from_timestamp_opt(secs, nanos).expect("invalid offset datetime")
}
