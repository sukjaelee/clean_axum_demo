use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime, PrimitiveDateTime};

// use this in dto timestamp fields
pub fn convert_naive_to_offset(value: NaiveDateTime) -> OffsetDateTime {
    OffsetDateTime::from_unix_timestamp(value.and_utc().timestamp()).unwrap()
}

// use this in updating timestamp fields
pub fn convert_offset_to_naive(value: OffsetDateTime) -> Option<DateTime<Utc>> {
    let secs = value.unix_timestamp();
    let nanos = value.nanosecond();
    DateTime::from_timestamp(secs, nanos)
}

// use this in inserting timestamp fields
pub fn convert_offset_to_primitive(value: Option<OffsetDateTime>) -> Option<PrimitiveDateTime> {
    value.map(|v| v.date().with_time(v.time()))
}

// OffsetDateTime
pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = date.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    OffsetDateTime::parse(&s, &Rfc3339).map_err(serde::de::Error::custom)
}

// support for Option<OffsetDateTime>
pub mod option {
    use super::*;
    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => super::serialize(d, serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => OffsetDateTime::parse(&s, &Rfc3339)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
