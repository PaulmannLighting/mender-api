use serde::{Deserialize, Serialize};

/// Available countries for devices in the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Country {
    /// Austria
    At,
    /// Germany
    De,
    /// Denmark
    Dk,
    /// France
    Fr,
    /// Romania
    Ro,
}
