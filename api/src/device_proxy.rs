use uuid::Uuid;

use crate::dto::{Attribute, Device, DeviceGroup, Tag};
use crate::session::Session;
use crate::{Deployments, Devices, Tags};

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

    /// Get the group of the specified device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn get_group(&self) -> reqwest::Result<DeviceGroup> {
        Devices::get_group(self.session, self.id).await
    }

    /// Add the device to the specified group.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn add_to_group<T>(&self, group_name: T) -> reqwest::Result<()>
    where
        T: ToString + Send,
    {
        Devices::set_group(self.session, self.id, group_name).await
    }

    /// Add tags to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn add_tags(&self, tags: &[Tag]) -> reqwest::Result<()> {
        Tags::add(self.session, self.id, tags).await
    }

    /// Add a tag to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn add_tag(&self, tag: Tag) -> reqwest::Result<()> {
        self.add_tags(&[tag]).await
    }

    /// Assign tags to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn assign_tags(&self, tags: &[Tag]) -> reqwest::Result<()> {
        Tags::assign(self.session, self.id, tags).await
    }

    /// Assign tags to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn clear_tags(&self) -> reqwest::Result<()> {
        Tags::clear(self.session, self.id).await
    }

    /// Abort any ongoing deployment for the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn abort_deployment(&self) -> reqwest::Result<()> {
        Deployments::abort_device(self.session, self.id).await
    }

    /// Get the tags assigned to the device.
    ///
    /// # Errors
    ///
    /// Return a [`reqwest::Error`] if the request fails.
    pub async fn tags(&self) -> reqwest::Result<Vec<Attribute>> {
        Ok(self.get().await?.tags().cloned().collect())
    }
}
