use serde::{Deserialize, Serialize};

/// Group payload for adding a device to a group.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Group {
    #[serde(rename = "group")]
    name: String,
}

impl Group {
    /// Creates a new `Group` instance.
    #[must_use]
    pub const fn new(name: String) -> Self {
        Self { name }
    }

    /// Returns the name of the group.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }
}
