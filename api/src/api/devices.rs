//! Devices management API.

use std::num::NonZero;

use proxy::Proxy;
use uuid::Uuid;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::dto::Device;
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

mod proxy;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Devices management API.
pub trait Devices<'this, 'path> {
    /// List devices.
    fn list(self, page_size: Option<NonZero<usize>>) -> PageIterator<'this, 'path, Device>;

    /// Collect devices into a `Vec`.
    fn collect(
        self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<Device>>> + Send;

    /// Return a proxy object to manage the device with the specified ID.
    fn device(self, id: Uuid) -> Proxy<'this>;
}

impl<'session> Devices<'session, 'session> for &'session Session {
    fn list(self, page_size: Option<NonZero<usize>>) -> PageIterator<'session, 'static, Device> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(self, page_size: Option<NonZero<usize>>) -> reqwest::Result<Vec<Device>> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }

    fn device(self, id: Uuid) -> Proxy<'session> {
        Proxy::new(self, id)
    }
}
