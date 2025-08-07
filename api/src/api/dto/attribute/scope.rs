use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Attribute scopes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    /// The attribute is specific to device ID.
    Identity,
    /// The attribute is specific to a system.
    System,
    /// The attribute is specific to an inventory.
    Inventory,
    /// The attribute is specific to tags.
    Tags,
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identity => write!(f, "identity"),
            Self::System => write!(f, "system"),
            Self::Inventory => write!(f, "inventory"),
            Self::Tags => write!(f, "tags"),
        }
    }
}
