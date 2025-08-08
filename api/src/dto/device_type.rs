use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Available device types in the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// Paulmann smik device.
    Paulmann,
}

impl Display for DeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Paulmann => write!(f, "paulmann"),
        }
    }
}
