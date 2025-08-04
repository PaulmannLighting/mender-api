//! Data structures for listing devices.

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api::dto::Attribute;

/// A list of devices in the Mender server.
pub type DeviceList = Vec<Device>;

/// A device in the Mender server.
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct Device {
    id: Uuid,
    attributes: Vec<Attribute>,
    updated_ts: DateTime<FixedOffset>,
}

impl Device {
    /// Create a new `Device` with the given ID, attributes, and updated timestamp.
    #[must_use]
    pub const fn new(
        id: Uuid,
        attributes: Vec<Attribute>,
        updated_ts: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            id,
            attributes,
            updated_ts,
        }
    }

    /// Return the ID of the device.
    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }

    /// Return the attributes of the device.
    #[must_use]
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }

    /// Return the updated timestamp of the device.
    #[must_use]
    pub const fn updated_ts(&self) -> &DateTime<FixedOffset> {
        &self.updated_ts
    }
}
