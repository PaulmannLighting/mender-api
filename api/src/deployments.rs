use std::num::NonZero;

use log::error;
use uuid::Uuid;

use crate::dto::{ListDeployment, NewDeployment, PutDeployment, Status};
use crate::pager::{DEFAULT_PAGE_SIZE, PageIterator, Pager};
use crate::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

/// Deployments management API.
pub trait Deployments {
    /// List deployment.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, ListDeployment>;

    /// Collect deployment into a `Vec`.
    fn collect(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<ListDeployment>>> + Send;

    /// List device of the given deployment.
    fn devices_of(&self, id: Uuid) -> impl Future<Output = reqwest::Result<Vec<Uuid>>> + Send;

    /// Create a new deployment.
    fn create(
        &self,
        deployment: &NewDeployment,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Abort a deployment.
    fn abort(&self, id: Uuid) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Abort all ongoing deployments.
    fn abort_all(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<()>> + Send;
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

    async fn abort(&self, id: Uuid) -> reqwest::Result<String> {
        self.client()
            .put(self.format_url(format!("{PATH}/{id}/status"), None))
            .bearer_auth(self.bearer_token())
            .json(&PutDeployment::new(Status::Aborted))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn abort_all(&self, page_size: Option<NonZero<usize>>) -> reqwest::Result<()> {
        let mut deployments = self.list(page_size);
        let mut last_error = None;

        while let Some(deployment) = deployments.next().await {
            if let Err(error) = self.abort(deployment?.id()).await {
                error!("Failed to abort deployment: {error}");
                last_error.replace(error);
            }
        }

        last_error.map_or_else(|| Ok(()), Err)
    }
}
