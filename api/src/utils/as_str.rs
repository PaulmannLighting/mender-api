//! De-/serialize types as strings.

use std::fmt::Display;
use std::str::FromStr;

use serde::de::Error;
use serde::{Deserialize, Deserializer, Serializer};

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr<Err: Display>,
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<T>()
        .map_err(Error::custom)
}

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: ToString,
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}
