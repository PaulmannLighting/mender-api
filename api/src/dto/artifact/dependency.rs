use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::dto::DeviceType;
use crate::utils::display_slice;

/// Artifact dependencies.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dependency {
    device_type: Vec<DeviceType>,
}

impl Dependency {
    /// Creates a new `Dependency` instance.
    #[must_use]
    pub const fn new(device_type: Vec<DeviceType>) -> Self {
        Self { device_type }
    }

    /// Returns the device types associated with the dependency.
    #[must_use]
    pub fn device_type(&self) -> &[DeviceType] {
        &self.device_type
    }
}

impl Display for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Dependency {{ device_types: ")?;
        display_slice(&self.device_type, f)?;
        write!(f, " }}")
    }
}
