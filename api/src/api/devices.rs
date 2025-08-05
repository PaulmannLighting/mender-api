//! Devices management API.

use crate::api::dto::DeviceList;
use crate::api::pager::Pager;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Devices management API.
pub trait Devices {
    /// List devices.
    fn list(&self) -> impl Future<Output = reqwest::Result<DeviceList>> + Send;
}

impl Devices for Session {
    async fn list(&self) -> reqwest::Result<DeviceList> {
        Pager::new(self, PATH).iter().await
    }
}
