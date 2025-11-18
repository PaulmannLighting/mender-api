use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::dto::DeviceType;

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
        let types: Vec<String> = self.device_type.iter().map(|dt| dt.to_string()).collect();
        write!(f, "Dependency {{ device_types: [")?;

        for (index, device_type) in self.device_type.iter().enumerate() {
            write!(f, "{device_type}")?;

            if index < types.len().saturating_sub(1) {
                write!(f, ", ")?;
            }
        }

        write!(f, "] }}")
    }
}
