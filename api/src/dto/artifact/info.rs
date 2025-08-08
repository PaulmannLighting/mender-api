use serde::{Deserialize, Serialize};

/// Information about an artifact.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Info {
    format: String, // TODO: use a specific enum for format
    version: u8,
}

impl Info {
    /// Creates a new `Info` instance.
    #[must_use]
    pub const fn new(format: String, version: u8) -> Self {
        Self { format, version }
    }

    /// Returns the format of the artifact.
    #[must_use]
    pub fn format(&self) -> &str {
        &self.format
    }

    /// Returns the version of the artifact.
    #[must_use]
    pub const fn version(&self) -> u8 {
        self.version
    }
}
