use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

/// File information for an artifact.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct File {
    name: String,
    checksum: String,
    size: usize,
    date: DateTime<FixedOffset>,
}

impl File {
    /// Creates a new `File` instance.
    #[must_use]
    pub const fn new(
        name: String,
        checksum: String,
        size: usize,
        date: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            name,
            checksum,
            size,
            date,
        }
    }

    /// Returns the name of the file.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the checksum of the file.
    #[must_use]
    pub fn checksum(&self) -> &str {
        &self.checksum
    }

    /// Returns the size of the file.
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }

    /// Returns the date of the file.
    #[must_use]
    pub const fn date(&self) -> DateTime<FixedOffset> {
        self.date
    }
}
