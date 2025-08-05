use std::num::NonZero;

use log::debug;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::dto::DeviceList;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

pub trait Deployments {
    /// Return the given page of deployments.
    fn page(
        &self,
        page_size: NonZero<usize>,
        page_no: NonZero<usize>,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Iterate over all pages with the specified page size,
    #[allow(async_fn_in_trait)]
    async fn iter_sized(&self, page_size: NonZero<usize>) -> reqwest::Result<String> {
        let mut devices = String::new();

        for page_no in (1..).filter_map(NonZero::new) {
            debug!("Fetching devices page #{page_no} with size {page_size}");
            let page = self.page(page_size, page_no).await?;

            if page.len() < page_size.get() {
                devices.push_str(&page);
                break;
            } else {
                devices.push_str(&page);
            }
        }

        Ok(devices)
    }

    /// Iterate over all pages with the default page size of 100.
    #[allow(async_fn_in_trait)]
    async fn iter(&self) -> reqwest::Result<String> {
        self.iter_sized(DEFAULT_PAGE_SIZE).await
    }
}

impl Deployments for Session {
    async fn page(
        &self,
        page_size: NonZero<usize>,
        page_no: NonZero<usize>,
    ) -> reqwest::Result<String> {
        let mut url = self.url(PATH);
        url.set_query(Some(&format!("per_page={page_size}&page={page_no}")));
        self.client()
            .get(url)
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
