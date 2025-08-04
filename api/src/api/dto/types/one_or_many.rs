use std::vec::IntoIter;

use serde::{Deserialize, Serialize};

/// A wrapper type to represent either a single item or a list of items.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum OneOrMany<T> {
    /// Represents a single item.
    One(T),
    /// Represents a list of items.
    Many(Vec<T>),
}

impl<T> OneOrMany<T> {
    /// Convert the `OneOrMany` into a `Vec<T>`.
    pub fn into_vec(self) -> Vec<T> {
        match self {
            OneOrMany::One(item) => vec![item],
            OneOrMany::Many(items) => items,
        }
    }
}

impl<T> From<T> for OneOrMany<T> {
    fn from(item: T) -> Self {
        OneOrMany::One(item)
    }
}

impl<T> From<Vec<T>> for OneOrMany<T> {
    fn from(items: Vec<T>) -> Self {
        OneOrMany::Many(items)
    }
}

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(one_or_many: OneOrMany<T>) -> Self {
        one_or_many.into_vec()
    }
}

impl<T> IntoIterator for OneOrMany<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_vec().into_iter()
    }
}
