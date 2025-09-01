//! Payload for deployment PUT requests.

use serde::Serialize;

use crate::dto::DeploymentStatus;

/// A new deployment put request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    status: DeploymentStatus,
}

impl Deployment {
    /// Creates a new `Deployment` instance.
    #[must_use]
    pub const fn new(status: DeploymentStatus) -> Self {
        Self { status }
    }
}
