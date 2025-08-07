use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

use crate::api::dto::Scope;

/// Attribute values.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Value<T> {
    #[serde(rename = "value")]
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

impl<T> AsRef<T> for Value<T> {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T> AsMut<T> for Value<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T> Deref for Value<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for Value<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> Display for Value<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Value: {}, Scope: {}", self.inner, self.scope)
    }
}
