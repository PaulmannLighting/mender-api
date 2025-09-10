use serde::Serialize;
use uuid::Uuid;

/// A new deployment request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct Deployment<'name, 'artifact, 'devices> {
    name: &'name str,
    artifact_name: &'artifact str,
    #[serde(skip_serializing_if = "Option::is_none")]
    devices: Option<&'devices [Uuid]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    all_devices: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retries: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autogenerate_delta: Option<bool>,
}

impl<'name, 'artifact, 'devices> Deployment<'name, 'artifact, 'devices> {
    /// Creates a new `Deployment` instance.
    #[must_use]
    pub const fn new(name: &'name str, artifact_name: &'artifact str) -> Self {
        Self {
            name,
            artifact_name,
            devices: None,
            all_devices: None,
            retries: None,
            autogenerate_delta: None,
        }
    }

    /// Set the devices for the deployment.
    #[must_use]
    pub const fn with_devices(mut self, devices: &'devices [Uuid]) -> Self {
        self.devices.replace(devices);
        self.all_devices = None;
        self
    }

    /// Set the deployment for all devices.
    #[must_use]
    pub const fn for_all_devices(mut self) -> Self {
        self.devices = None;
        self.all_devices.replace(true);
        self
    }

    /// Set the number of retries for the deployment.
    #[must_use]
    pub const fn with_retries(mut self, retries: usize) -> Self {
        self.retries.replace(retries);
        self
    }

    /// Set whether to autogenerate delta updates for the deployment.
    #[must_use]
    pub const fn with_autogenerate_delta(mut self, autogenerate: bool) -> Self {
        self.autogenerate_delta.replace(autogenerate);
        self
    }
}
