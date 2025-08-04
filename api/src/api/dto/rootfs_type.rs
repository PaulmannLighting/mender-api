use serde::{Deserialize, Serialize};

/// Available root filesystem types for devices.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum RootfsType {
    /// Root filesystem type is ext4.
    Ext4,
}
