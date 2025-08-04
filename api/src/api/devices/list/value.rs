use serde::{Deserialize, Serialize};

use super::scope::Scope;

/// Attribute values.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Value<T> {
    value: T,
    scope: Scope,
    description: Option<String>,
}
