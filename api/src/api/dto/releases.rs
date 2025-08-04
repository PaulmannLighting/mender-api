//! Types to parse the releases API response.

use serde::{Deserialize, Serialize};

use crate::api::dto::Artifact;

/// List of releases.
pub type ReleaseList = Vec<Release>;

/// Represents a release in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Release {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Artifacts")]
    artifacts: Vec<Artifact>,
}

impl Release {
    /// Creates a new `Release` instance.
    #[must_use]
    pub const fn new(name: String, artifacts: Vec<Artifact>) -> Self {
        Self { name, artifacts }
    }

    /// Returns the name of the release.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the artifacts associated with the release.
    #[must_use]
    pub fn artifacts(&self) -> &[Artifact] {
        &self.artifacts
    }
}
