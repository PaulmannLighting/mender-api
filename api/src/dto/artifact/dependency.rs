use serde::{Deserialize, Serialize};

use crate::dto::DeviceType;

/// Artifact dependencies.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
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
