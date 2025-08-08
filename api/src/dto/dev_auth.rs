use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::{AuthentificationSet, Identity, Status};

/// Represents a device from the device authentication API in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Device {
    id: Uuid,
    #[serde(rename = "identity_data")]
    identity: Identity,
    status: Status,
    decommissioning: bool,
    #[serde(rename = "created_ts")]
    created: DateTime<FixedOffset>,
    #[serde(rename = "updated_ts")]
    updated: DateTime<FixedOffset>,
    auth_sets: Vec<AuthentificationSet>,
}

impl Device {
    /// Returns the ID of the device.
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the identity of the device.
    #[must_use]
    pub const fn identity(&self) -> &Identity {
        &self.identity
    }

    /// Returns the status of the device.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Returns whether the device is decommissioning.
    #[must_use]
    pub const fn decommissioning(&self) -> bool {
        self.decommissioning
    }

    /// Returns the creation timestamp of the device.
    #[must_use]
    pub const fn created(&self) -> DateTime<FixedOffset> {
        self.created
    }

    /// Returns the last updated timestamp of the device.
    #[must_use]
    pub const fn updated(&self) -> DateTime<FixedOffset> {
        self.updated
    }

    /// Returns the authentication sets associated with the device.
    #[must_use]
    pub fn auth_sets(&self) -> &[AuthentificationSet] {
        &self.auth_sets
    }
}
