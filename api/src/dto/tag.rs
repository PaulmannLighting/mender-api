use serde::{Deserialize, Serialize};

/// Represents a tag of a device.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    name: String,
    value: String,
    description: Option<String>,
}

impl Tag {
    /// Create a new `Tag` with the specified name, value, and optional description.
    #[must_use]
    pub const fn new(name: String, value: String, description: Option<String>) -> Self {
        Self {
            name,
            value,
            description,
        }
    }

    /// Return the name of the tag.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Return the value of the tag.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return the optional description of the tag.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
