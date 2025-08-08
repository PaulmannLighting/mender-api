use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};

use crate::utils::as_str;

/// Represents the identity of a device in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Identity {
    #[serde(with = "as_str")]
    mac: MacAddr6,
}

impl Identity {
    /// Return the MAC address of the device.
    #[must_use]
    pub const fn mac(&self) -> MacAddr6 {
        self.mac
    }
}

impl From<Identity> for MacAddr6 {
    fn from(identity: Identity) -> Self {
        identity.mac
    }
}
