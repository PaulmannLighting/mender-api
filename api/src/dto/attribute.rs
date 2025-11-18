use std::fmt::Display;

use serde::{Deserialize, Serialize};

pub use self::known_attribute::KnownAttribute;
pub use self::unknown_attribute::UnknownAttribute;
use crate::dto::scope::Scope;

mod known_attribute;
mod unknown_attribute;

/// Available attributes for device in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Attribute {
    /// Known attributes with known types.
    Known(KnownAttribute),
    /// Unknown attributes with unknown types.
    Unknown(UnknownAttribute),
}

impl Attribute {
    /// Return the attribute's scope.
    #[must_use]
    pub const fn scope(&self) -> Scope {
        match self {
            Self::Known(known) => known.scope(),
            Self::Unknown(unknown) => unknown.scope(),
        }
    }
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(known) => Display::fmt(known, f),
            Self::Unknown(unknown) => Display::fmt(unknown, f),
        }
    }
}
