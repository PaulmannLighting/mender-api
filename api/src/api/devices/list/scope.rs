use serde::{Deserialize, Serialize};

/// Attribute scopes.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scope {
    Identity,
    System,
    Inventory,
}
