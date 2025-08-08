//! Devices management API.

use std::num::NonZero;

use uuid::Uuid;

use crate::dto::{Device, DeviceGroup};
use crate::pager::{DEFAULT_PAGE_SIZE, PageIterator, Pager};
use crate::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Devices management API.
pub trait Devices {
    /// List device.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, Device>;

    /// Collect device into a `Vec`.
    fn collect(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<Device>>> + Send;

    /// Return the status of the device.
    fn status(&self, id: Uuid) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Add the device to the specified group.
    fn add_to_group<T>(
        &self,
        id: Uuid,
        group_name: T,
    ) -> impl Future<Output = reqwest::Result<String>> + Send
    where
        T: AsRef<str> + Send;
}

impl Devices for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, Device> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(&self, page_size: Option<NonZero<usize>>) -> reqwest::Result<Vec<Device>> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }

    async fn status(&self, id: Uuid) -> reqwest::Result<String> {
        self.client()
            .get(self.format_url(format!("{PATH}/{id}/status"), None))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn add_to_group<T>(&self, id: Uuid, group_name: T) -> reqwest::Result<String>
    where
        T: AsRef<str> + Send,
    {
        self.client()
            .post(self.format_url(format!("{PATH}/{id}/group"), None))
            .bearer_auth(self.bearer_token())
            .json(&DeviceGroup::new(group_name.as_ref()))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
