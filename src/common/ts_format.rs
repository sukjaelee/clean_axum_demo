use serde::{self, Deserialize, Deserializer, Serializer};
use time::{format_description, OffsetDateTime, PrimitiveDateTime};

/// This module provides serialization and deserialization functions for `OffsetDateTime`
/// using a specific format. The format is defined as a constant string.
/// Define the desired time format.
const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

// OffsetDateTime
pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description::parse(FORMAT).map_err(serde::ser::Error::custom)?;
    let s = date.format(&format).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;
    PrimitiveDateTime::parse(&s, &format)
        .map(|dt| dt.assume_utc())
        .map_err(serde::de::Error::custom)
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
            Some(s) => {
                let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;
                PrimitiveDateTime::parse(&s, &format)
                    .map(|dt| Some(dt.assume_utc()))
                    .map_err(serde::de::Error::custom)
            }
            None => Ok(None),
        }
    }
}
