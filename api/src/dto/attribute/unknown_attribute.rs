use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::dto::Scope;

/// Unknown attributes for devices in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct UnknownAttribute {
    name: String,
    value: String,
    description: Option<String>,
    scope: Scope,
}

impl UnknownAttribute {
    /// Create a new attribute.
    #[must_use]
    pub const fn new(
        name: String,
        value: String,
        description: Option<String>,
        scope: Scope,
    ) -> Self {
        Self {
            name,
            value,
            description,
            scope,
        }
    }

    /// Return the name of the attribute.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the value of the attribute.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Return the description of the attribute, if any.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Return the scope of the attribute.
    #[must_use]
    pub const fn scope(&self) -> Scope {
        self.scope
    }
}

impl Display for UnknownAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(desc) = &self.description {
            write!(
                f,
                "{}: {} ({}) [{}]",
                self.name, self.value, desc, self.scope
            )
        } else {
            write!(f, "{}: {} [{}]", self.name, self.value, self.scope)
        }
    }
}
