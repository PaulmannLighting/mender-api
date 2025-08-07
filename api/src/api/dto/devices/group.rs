use serde::Serialize;

/// Group payload for adding a device to a group.
#[derive(Clone, Debug, Serialize)]
pub struct Group<'name> {
    #[serde(rename = "group")]
    name: &'name str,
}

impl<'name> Group<'name> {
    /// Creates a new `Group` instance.
    #[must_use]
    pub const fn new(name: &'name str) -> Self {
        Self { name }
    }

    /// Returns the name of the group.
    #[must_use]
    pub const fn name(&self) -> &str {
        self.name
    }
}
