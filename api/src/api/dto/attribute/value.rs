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
}

impl<T> Value<T> {
    /// Create a new `Value` with the given value, scope, and optional description.
    #[must_use]
    pub const fn new(inner: T, scope: Scope) -> Self {
        Self { inner, scope }
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

    /// Display the value with the representation of the inner value given by `fmt_inner`.
    pub(crate) fn display_with<F>(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        fmt_inner: F,
    ) -> std::fmt::Result
    where
        F: FnOnce(&T, &mut std::fmt::Formatter<'_>) -> std::fmt::Result,
    {
        write!(f, "Value: ")?;
        fmt_inner(&self.inner, f)?;
        write!(f, ", Scope: {}", self.scope)
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
        self.display_with(f, Display::fmt)
    }
}
