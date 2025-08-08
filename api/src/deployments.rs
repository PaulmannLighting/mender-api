use std::num::NonZero;

use uuid::Uuid;

use crate::dto::{ListDeployment, NewDeployment};
use crate::pager::{DEFAULT_PAGE_SIZE, PageIterator, Pager};
use crate::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

/// Deployments management API.
pub trait Deployments {
    /// List deployments.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, ListDeployment>;

    /// Collect deployments into a `Vec`.
    fn collect(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<ListDeployment>>> + Send;

    /// List devices of the given deployment.
    fn devices_of(&self, id: Uuid) -> impl Future<Output = reqwest::Result<Vec<Uuid>>> + Send;

    /// Create a new deployment.
    fn create(
        &self,
        deployment: &NewDeployment,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl Deployments for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, ListDeployment> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> reqwest::Result<Vec<ListDeployment>> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }

    async fn devices_of(&self, id: Uuid) -> reqwest::Result<Vec<Uuid>> {
        self.client()
            .get(self.format_url(format!("{PATH}/{id}/device_list"), None))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    async fn create(&self, deployment: &NewDeployment) -> reqwest::Result<String> {
        self.client()
            .post(self.format_url(PATH, None))
            .bearer_auth(self.bearer_token())
            .json(deployment)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
