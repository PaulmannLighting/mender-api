use serde::{Deserialize, Serialize};

use super::{File, TypeInfo};

/// Update information of an artifact.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Update {
    type_info: TypeInfo,
    files: Vec<File>,
}

impl Update {
    /// Creates a new `Update` instance.
    #[must_use]
    pub const fn new(type_info: TypeInfo, files: Vec<File>) -> Self {
        Self { type_info, files }
    }

    /// Returns the type information of the update.
    #[must_use]
    pub const fn type_info(&self) -> &TypeInfo {
        &self.type_info
    }

    /// Returns the files associated with the update.
    #[must_use]
    pub fn files(&self) -> &[File] {
        &self.files
    }
}
