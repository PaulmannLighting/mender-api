use serde::{Deserialize, Serialize};

/// A wrapper type to represent either a single item or a list of items.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
