use serde::{Deserialize, Serialize};

/// Available device types in the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum DeviceType {
    /// Paulmann smik device.
    Paulmann,
}
