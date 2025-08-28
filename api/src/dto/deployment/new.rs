use serde::Serialize;
use uuid::Uuid;

/// A new deployment request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment<'name, 'artifact, 'devices> {
    name: &'name str,
    artifact_name: &'artifact str,
    #[serde(skip_serializing_if = "<[Uuid]>::is_empty")]
    devices: &'devices [Uuid],
    retries: usize,
}

impl<'name, 'artifact, 'devices> Deployment<'name, 'artifact, 'devices> {
    /// Creates a new `Deployment` instance.
    #[must_use]
    pub const fn new(
        name: &'name str,
        artifact_name: &'artifact str,
        devices: &'devices [Uuid],
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
