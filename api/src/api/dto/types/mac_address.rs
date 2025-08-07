use std::fmt::Display;
use std::str::FromStr;

use macaddr::MacAddr6;
use serde::{Deserialize, Deserializer, Serialize};

/// MAC address wrapper to allow de-/serialization from/to string.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct MacAddress(MacAddr6);

impl MacAddress {
    /// Return the inner MAC address.
    #[must_use]
    pub const fn into_inner(self) -> MacAddr6 {
        self.0
    }
}

impl From<MacAddr6> for MacAddress {
    fn from(mac: MacAddr6) -> Self {
        Self(mac)
    }
}

impl From<MacAddress> for MacAddr6 {
    fn from(mac: MacAddress) -> Self {
        mac.0
    }
}

impl<'de> Deserialize<'de> for MacAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer).and_then(|string| {
            MacAddr6::from_str(&string)
                .map(MacAddress)
                .map_err(serde::de::Error::custom)
        })
    }
}

impl Serialize for MacAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
