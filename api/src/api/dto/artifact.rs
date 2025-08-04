use std::collections::BTreeMap;

use chrono::{DateTime, FixedOffset};
pub use dependency::Dependency;
pub use file::File;
pub use info::Info;
use serde::{Deserialize, Serialize};
pub use type_info::TypeInfo;
pub use update::Update;
use uuid::Uuid;

use crate::api::dto::DeviceType;

mod dependency;
mod file;
mod info;
mod type_info;
mod update;

/// Represents an artifact in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct Artifact {
    id: Uuid,
    description: Option<String>,
    name: String,
    #[serde(rename = "device_types_compatible")]
    compatible_device_types: Vec<DeviceType>,
    info: Info,
    signed: bool,
    updates: Vec<Update>,
    #[serde(
        rename = "artifact_provides",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    provides: BTreeMap<String, String>,
    #[serde(rename = "artifact_depends")]
    depends: Dependency,
    #[serde(
        rename = "clears_artifact_provides",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    clears_provides: Vec<String>,
    size: usize,
    modified: DateTime<FixedOffset>,
}

impl Artifact {
    /// Creates a new `Artifact` instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        id: Uuid,
        description: Option<String>,
        name: String,
        compatible_device_types: Vec<DeviceType>,
        info: Info,
        signed: bool,
        updates: Vec<Update>,
        provides: BTreeMap<String, String>,
        depends: Dependency,
        clears_provides: Vec<String>,
        size: usize,
        modified: DateTime<FixedOffset>,
    ) -> Self {
        Self {
            id,
            description,
            name,
            compatible_device_types,
            info,
            signed,
            updates,
            provides,
            depends,
            clears_provides,
            size,
            modified,
        }
    }

    /// Returns the ID of the artifact.
    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the description of the artifact.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    /// Returns the name of the artifact.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the compatible device types for the artifact.
    #[must_use]
    pub fn compatible_device_types(&self) -> &[DeviceType] {
        &self.compatible_device_types
    }

    /// Returns the information associated with the artifact.
    #[must_use]
    pub const fn info(&self) -> &Info {
        &self.info
    }

    /// Returns whether the artifact is signed.
    #[must_use]
    pub const fn signed(&self) -> bool {
        self.signed
    }

    /// Returns the updates associated with the artifact.
    #[must_use]
    pub fn updates(&self) -> &[Update] {
        &self.updates
    }

    /// Returns the provides map of the artifact.
    #[must_use]
    pub fn provides(&self) -> &BTreeMap<String, String> {
        &self.provides
    }

    /// Returns the dependencies of the artifact.
    #[must_use]
    pub fn depends(&self) -> &Dependency {
        &self.depends
    }

    /// Returns the clears provides list of the artifact.
    #[must_use]
    pub fn clears_provides(&self) -> &[String] {
        &self.clears_provides
    }

    /// Returns the size of the artifact in bytes.
    #[must_use]
    pub const fn size(&self) -> usize {
        self.size
    }
}
