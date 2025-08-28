use serde::Serialize;
use uuid::Uuid;

/// A new deployment request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    name: String,
    artifact_name: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    devices: Vec<Uuid>,
    retries: usize,
}

impl Deployment {
    /// Creates a new `Deployment` instance.
    #[must_use]
    pub const fn new(
        name: String,
        artifact_name: String,
        devices: Vec<Uuid>,
        retries: usize,
    ) -> Self {
        Self {
            name,
            artifact_name,
            devices,
            retries,
        }
    }
}
