use serde::{self, Deserialize, Deserializer, Serializer};
use time::{format_description, OffsetDateTime};

// Define the desired time format.
const FORMAT: &str = "[year]-[month]-[day] [hour]:[minute]:[second]";

pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let format = format_description::parse(FORMAT).map_err(serde::ser::Error::custom)?;
    let s = date.format(&format).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&s)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let format = format_description::parse(FORMAT).map_err(serde::de::Error::custom)?;
    OffsetDateTime::parse(&s, &format).map_err(serde::de::Error::custom)
}
