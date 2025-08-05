//! Devices management API.

use crate::api::dto::Device;
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Devices management API.
pub trait Devices {
    /// List devices.
    fn list(&self) -> PageIterator<Device>;

    /// Collect devices into a `Vec`.
    fn collect(&self) -> impl Future<Output = reqwest::Result<Vec<Device>>> + Send;
}

impl Devices for Session {
    fn list(&self) -> PageIterator<Device> {
        Pager::new(self, PATH).into()
    }

    async fn collect(&self) -> reqwest::Result<Vec<Device>> {
        Pager::new(self, PATH).collect().await
    }
}
