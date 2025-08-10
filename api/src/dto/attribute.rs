use std::fmt::Display;

pub use known_attribute::KnownAttribute;
use serde::{Deserialize, Serialize};

use crate::dto::scope::Scope;

mod known_attribute;

/// Available attributes for device in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Attribute {
    /// Known attributes with known types.
    Known(KnownAttribute),
    /// Unknown attributes with unknown types.
    Unknown {
        /// The name of the unknown attribute.
        name: String,
        /// The value of the unknown attribute.
        value: String,
        /// An optional description.
        description: Option<String>,
        /// The scope of this attribute.
        scope: Scope,
    },
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(known) => Display::fmt(known, f),
            Self::Unknown {
                name,
                value,
                description,
                scope,
            } => {
                if let Some(desc) = description {
                    write!(f, "{name}: {value} ({desc}) [{scope}]")
                } else {
                    write!(f, "{name}: {value} [{scope}]")
                }
            }
        }
    }
}
