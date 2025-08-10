use uuid::Uuid;

use crate::dto::{Device, Tag};
use crate::session::Session;
use crate::{Devices, Tags};

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
}

impl DeviceProxy<'_> {
    /// Get the device details from the Mender server.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn get(&self) -> reqwest::Result<Device> {
        Devices::get(self.session, self.id).await
    }

    /// Add the device to the specified group.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn add_to_group<T>(&self, group_name: T) -> reqwest::Result<String>
    where
        T: AsRef<str> + Send,
    {
        Devices::add_to_group(self.session, self.id, group_name).await
    }

    /// Add tags to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn add_tags(&self, tags: &[Tag]) -> reqwest::Result<String> {
        Tags::add(self.session, self.id, tags).await
    }

    /// Assign tags to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn assign_tags(&self, tags: &[Tag]) -> reqwest::Result<String> {
        Tags::assign(self.session, self.id, tags).await
    }

    /// Assign tags to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn clear_tags(&self) -> reqwest::Result<String> {
        Tags::clear(self.session, self.id).await
    }
}
