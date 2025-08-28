use serde::{Deserialize, Serialize};

/// Group payload for adding a device to a group.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Group<T> {
    #[serde(rename = "group")]
    name: T,
}

impl<T> Group<T>
where
    T: AsRef<str>,
{
    /// Creates a new `Group` instance.
    #[must_use]
    pub const fn new(name: T) -> Self {
        Self { name }
    }

    /// Returns the name of the group.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Consumes the `Group` and returns its name.
    #[must_use]
    pub fn into_name(self) -> T {
        self.name
    }
}
