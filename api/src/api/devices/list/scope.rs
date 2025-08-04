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
}
