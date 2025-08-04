use serde::{Deserialize, Serialize};

use crate::api::dto::Scope;

/// Attribute values.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Value<T> {
    value: T,
    scope: Scope,
    description: Option<String>,
}

impl<T> Value<T> {
    /// Create a new `Value` with the given value, scope, and optional description.
    #[must_use]
    pub const fn new(value: T, scope: Scope, description: Option<String>) -> Self {
        Self {
            value,
            scope,
            description,
        }
    }

    /// Return the inner value.
    #[must_use]
    pub const fn value(&self) -> &T {
        &self.value
    }

    /// Return the scope of the value.
    #[must_use]
    pub const fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Return the description of the value.
    #[must_use]
    pub fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
