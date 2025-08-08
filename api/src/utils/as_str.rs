//! De-/serialize types as strings.

use std::fmt::Display;
use std::str::FromStr;

use serde::Deserialize;

pub(crate) fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr<Err: Display>,
    D: serde::Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    string.parse::<T>().map_err(serde::de::Error::custom)
}

pub(crate) fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}
