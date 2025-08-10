use uuid::Uuid;

use crate::Devices;
use crate::dto::Device;
use crate::session::Session;

/// A proxy for a device in the Mender server.
pub struct DeviceProxy<'session> {
    session: &'session Session,
    id: Uuid,
}

impl<'session> DeviceProxy<'session> {
    /// Create a new `Proxy` for the specified device ID and session.
    #[must_use]
    pub(crate) const fn new(session: &'session Session, id: Uuid) -> Self {
        Self { session, id }
    }

    /// Return the session associated with this device proxy.
    #[must_use]
    pub(crate) const fn session(&self) -> &Session {
        self.session
    }

    /// Return the ID of the device.
    #[must_use]
    pub(crate) const fn id(&self) -> Uuid {
        self.id
    }
}

impl DeviceProxy<'_> {
    /// Get the device details from the Mender server.
    pub async fn get(&self) -> reqwest::Result<Device> {
        Devices::get(self.session, self.id).await
    }

    /// Add the device to the specified group.
    pub async fn add_to_group<T>(&self, group_name: T) -> reqwest::Result<String>
    where
        T: AsRef<str> + Send,
    {
        Devices::add_to_group(self.session, self.id, group_name).await
    }
}
