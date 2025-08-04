//! Devices management API.

mod list;

use std::num::NonZero;

pub use list::{
    AsStr, Attribute, BootloaderIntegration, Country, Device, DeviceList, DeviceType, MacAddress,
    OneOrMany, RootfsType, Scope, Status, Value,
};
use log::debug;
use serde_json::json;

use crate::api::session::Session;

const DEFAULT_PAGE_SIZE: NonZero<usize> =
    NonZero::new(500).expect("Default page should be be non-zero.");
const PATH: &str = "/api/management/v1/inventory/devices";

/// Inventory API.
pub trait Devices {
    /// Return the devices API interface.
    fn page(
        &self,
        page_size: NonZero<usize>,
        page_no: NonZero<usize>,
    ) -> impl Future<Output = reqwest::Result<DeviceList>> + Send;

    /// Iterate over all pages with the specified page size,
    #[allow(async_fn_in_trait)]
    async fn iter_sized(&self, page_size: NonZero<usize>) -> reqwest::Result<DeviceList> {
        let mut devices = DeviceList::new();

        for page_no in (1..).filter_map(NonZero::new) {
            debug!("Fetching devices page #{page_no} with size {page_size}");
            let page = self.page(page_size, page_no).await?;

            if page.len() < page_size.get() {
                devices.extend(page);
                break;
            } else {
                devices.extend(page);
            }
        }

        Ok(devices)
    }

    /// Iterate over all pages with the default page size of 100.
    #[allow(async_fn_in_trait)]
    async fn iter(&self) -> reqwest::Result<DeviceList> {
        self.iter_sized(DEFAULT_PAGE_SIZE).await
    }
}

impl Devices for Session {
    async fn page(
        &self,
        page_size: NonZero<usize>,
        page_no: NonZero<usize>,
    ) -> reqwest::Result<DeviceList> {
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
}
