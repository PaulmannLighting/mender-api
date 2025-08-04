//! Devices management API.

mod list;

pub use list::{
    AsStr, Attribute, BootloaderIntegration, Country, Device, DeviceList, DeviceType, MacAddress,
    OneOrMany, RootfsType, Scope, Status, Value,
};
use serde_json::json;

use crate::api::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Inventory API.
pub trait Devices {
    /// Return the devices API interface.
    fn page(
        &self,
        page_size: usize,
        page_no: usize,
    ) -> impl Future<Output = reqwest::Result<DeviceList>>;

    /// Return the groups API interface.
    fn groups(&self) -> reqwest::Url;
}
impl Devices for Session {
    async fn page(&self, page_size: usize, page_no: usize) -> reqwest::Result<DeviceList> {
        let mut url = self.url(PATH);
        url.set_query(Some(&format!("per_page={page_size}&page={page_no}")));
        self.client()
            .get(url)
            .bearer_auth(self.bearer_token())
            .json(&json!({}))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    fn groups(&self) -> reqwest::Url {
        todo!()
    }
}
