use std::fmt::Display;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// A wrapper type to represent a type as a string for serialization and deserialization.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct AsStr<T>(T);

impl<T> AsStr<T> {
    /// Return the inner value.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> From<T> for AsStr<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<'de, T> Deserialize<'de> for AsStr<T>
where
    T: FromStr<Err: Display>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let string = String::deserialize(deserializer)?;
        string
            .parse::<T>()
            .map(AsStr)
            .map_err(serde::de::Error::custom)
    }
}

impl<T> Serialize for AsStr<T>
where
    T: ToString,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<T> Display for AsStr<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
