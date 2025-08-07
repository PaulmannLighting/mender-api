use serde::{Deserialize, Serialize};

use crate::api::dto::Scope;

/// Attribute values.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Value<T> {
    inner: T,
    scope: Scope,
    description: Option<String>,
}

impl<T> Value<T> {
    /// Create a new `Value` with the given value, scope, and optional description.
    #[must_use]
    pub const fn new(inner: T, scope: Scope, description: Option<String>) -> Self {
        Self {
            inner,
            scope,
            description,
        }
    }

    /// Return the inner value.
    #[must_use]
    pub const fn inner(&self) -> &T {
        &self.inner
    }

    /// Return the scope of the value.
    #[must_use]
    pub const fn scope(&self) -> &Scope {
        &self.scope
    }

    /// Return the description of the value.
    #[must_use]
    pub const fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
