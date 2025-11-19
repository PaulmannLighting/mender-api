//! Devices management API.

use std::num::NonZero;

use uuid::Uuid;

use crate::device_proxy::DeviceProxy;
use crate::dto::{Device, DeviceGroup};
use crate::pagination::{DEFAULT_PAGE_SIZE, Pager, Pages, PaginatedIterator};
use crate::session::Session;
use crate::utils::ResponseExt;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Devices management API.
pub trait Devices {
    /// Iterate over device pages.
    fn pages(&self, page_size: Option<NonZero<usize>>) -> Pages<'_, '_, Device>;

    /// List device.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Device>;

    /// Collect device into a `Vec`.
    fn collect(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<Device>>> + Send;

    /// Get a specific device by its ID.
    fn get(&self, id: Uuid) -> impl Future<Output = reqwest::Result<Device>> + Send;

    /// Get the group of the specified device.
    fn get_group(&self, id: Uuid) -> impl Future<Output = reqwest::Result<DeviceGroup>> + Send;

    /// Add the device to the specified group.
    fn set_group<T>(
        &self,
        id: Uuid,
        group_name: T,
    ) -> impl Future<Output = reqwest::Result<()>> + Send
    where
        T: ToString + Send;

    /// Return a device proxy for the specified device ID.
    fn proxy(&self, id: Uuid) -> DeviceProxy<'_>;
}

impl Devices for Session {
    fn pages(&self, page_size: Option<NonZero<usize>>) -> Pages<'_, '_, Device> {
        Pager::new(self, PATH.into(), page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Device> {
        Pager::new(self, PATH.into(), page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(&self, page_size: Option<NonZero<usize>>) -> reqwest::Result<Vec<Device>> {
        Pager::new(self, PATH.into(), page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }

    async fn get(&self, id: Uuid) -> reqwest::Result<Device> {
        self.get(format!("{PATH}/{id}"), None)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    async fn get_group(&self, id: Uuid) -> reqwest::Result<DeviceGroup> {
        self.get(format!("{PATH}/{id}/group"), None)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    async fn set_group<T>(&self, id: Uuid, group_name: T) -> reqwest::Result<()>
    where
        T: ToString + Send,
    {
        self.put(format!("{PATH}/{id}/group"), None)
            .json(&DeviceGroup::new(group_name.to_string()))
            .send()
            .await?
            .error_for_status()?
            .ensure_empty()
            .await
    }

    fn proxy(&self, id: Uuid) -> DeviceProxy<'_> {
        DeviceProxy::new(self, id)
    }
}
