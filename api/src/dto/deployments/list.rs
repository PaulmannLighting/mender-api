use chrono::{DateTime, FixedOffset};
use serde::Deserialize;
use uuid::Uuid;

use crate::dto::Kind;
use crate::dto::deployments::Status;

/// Represents a deployment in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    name: String,
    artifact_name: String,
    created: DateTime<FixedOffset>,
    finished: Option<DateTime<FixedOffset>>,
    id: Uuid,
    artifacts: Vec<Uuid>,
    status: Status,
    device_count: usize,
    max_devices: usize,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    groups: Vec<String>,
    #[serde(rename = "type")]
    kind: Kind,
}

impl Deployment {
    /// Returns the name of the deployment.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the artifact name associated with the deployment.
    #[must_use]
    pub fn artifact_name(&self) -> &str {
        &self.artifact_name
    }

    /// Returns the creation date of the deployment.
    #[must_use]
    pub const fn created(&self) -> DateTime<FixedOffset> {
        self.created
    }

    /// Returns the finish date of the deployment.
    #[must_use]
    pub const fn finished(&self) -> Option<DateTime<FixedOffset>> {
        self.finished
    }

    /// Returns the ID of the deployment.
    #[must_use]
    pub const fn id(&self) -> &Uuid {
        &self.id
    }

    /// Returns the list of artifact IDs associated with the deployment.
    #[must_use]
    pub fn artifacts(&self) -> &[Uuid] {
        &self.artifacts
    }

    /// Returns the status of the deployment.
    #[must_use]
    pub const fn status(&self) -> &Status {
        &self.status
    }

    /// Returns the number of devices in the deployment.
    #[must_use]
    pub const fn device_count(&self) -> usize {
        self.device_count
    }

    /// Returns the maximum number of devices allowed in the deployment.
    #[must_use]
    pub const fn max_devices(&self) -> usize {
        self.max_devices
    }

    /// Returns the groups associated with the deployment.
    #[must_use]
    pub fn groups(&self) -> &[String] {
        &self.groups
    }

    /// Returns the kind of the deployment.
    #[must_use]
    pub const fn kind(&self) -> &Kind {
        &self.kind
    }
}
