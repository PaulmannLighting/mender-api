//! Data structures for listing device.

use std::fmt::{Debug, Display};

use chrono::{DateTime, FixedOffset};
pub use group::Group;
use macaddr::MacAddr6;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::{Attribute, KnownAttribute};

mod group;

/// A device in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
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

    /// Return an iterator over known attributes of the device.
    pub fn known_attributes(&self) -> impl Iterator<Item = &KnownAttribute> {
        self.attributes.iter().filter_map(|attr| {
            if let Attribute::Known(known_attr) = attr {
                Some(known_attr)
            } else {
                None
            }
        })
    }

    /// Return the MAC addresses of the device.
    pub fn mac_addresses(&self) -> impl Iterator<Item = MacAddr6> {
        self.known_attributes().filter_map(|attr| {
            if let KnownAttribute::Mac { value, .. } = attr {
                Some(*value)
            } else {
                None
            }
        })
    }

    /// Return the MAC address of the device if it exists.
    #[must_use]
    pub fn mac_address(&self) -> Option<MacAddr6> {
        self.mac_addresses().next()
    }

    /// Return the groups of the device.
    pub fn groups(&self) -> impl Iterator<Item = &str> {
        self.known_attributes().filter_map(|attr| {
            if let KnownAttribute::Group { value, .. } = attr {
                Some(value.as_str())
            } else {
                None
            }
        })
    }
}

impl Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Device: {}\n\t- updated: {}\n\t- attributes:",
            self.id, self.updated_ts
        )?;

        for attribute in &self.attributes {
            write!(f, "\t\t- ")?;
            Display::fmt(attribute, f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}
