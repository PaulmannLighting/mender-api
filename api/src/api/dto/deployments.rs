//! Deployment-related data structures and types.

use chrono::{DateTime, FixedOffset};
pub use kind::Kind;
use serde::{Deserialize, Serialize};
pub use status::Status;
use uuid::Uuid;

mod kind;
mod status;

/// List of deployments.
pub type DeploymentList = Vec<Deployment>;

/// Represents a deployment in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
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
    /// Creates a new `Deployment` instance.
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        name: String,
        artifact_name: String,
        created: DateTime<FixedOffset>,
        finished: Option<DateTime<FixedOffset>>,
        id: Uuid,
        artifacts: Vec<Uuid>,
        status: Status,
        device_count: usize,
        max_devices: usize,
        groups: Vec<String>,
        kind: Kind,
    ) -> Self {
        Self {
            name,
            artifact_name,
            created,
            finished,
            id,
            artifacts,
            status,
            device_count,
            max_devices,
            groups,
            kind,
        }
    }

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
    pub fn created(&self) -> DateTime<FixedOffset> {
        self.created
    }

    /// Returns the finish date of the deployment.
    #[must_use]
    pub fn finished(&self) -> Option<DateTime<FixedOffset>> {
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
