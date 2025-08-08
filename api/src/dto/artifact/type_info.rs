use serde::{Deserialize, Serialize};

/// Type information for artifacts in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TypeInfo {
    r#type: String,
}

impl TypeInfo {
    /// Creates a new `TypeInfo` instance.
    #[must_use]
    pub const fn new(r#type: String) -> Self {
        Self { r#type }
    }

    /// Returns the type of the artifact.
    #[must_use]
    pub fn typ(&self) -> &str {
        &self.r#type
    }
}
