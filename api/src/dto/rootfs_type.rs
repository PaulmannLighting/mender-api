use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Available root filesystem types for devices.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RootfsType {
    /// Root filesystem type is ext4.
    Ext4,
}

impl Display for RootfsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ext4 => write!(f, "ext4"),
        }
    }
}
