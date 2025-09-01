use std::fmt::Display;
use std::num::NonZero;

use log::error;
use uuid::Uuid;

use crate::dto::{ListDeployment, NewDeployment, PutDeployment, Status};
use crate::paging::{DEFAULT_PAGE_SIZE, PageIterator, Pager};
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
    fn create<N, A>(
        &self,
        name: N,
        artifact_name: A,
        devices: &[Uuid],
        retries: usize,
    ) -> impl Future<Output = reqwest::Result<String>> + Send
    where
        N: AsRef<str> + Send + Sync,
        A: AsRef<str> + Send + Sync;

    /// Create a new deployment for a group of devices.
    fn create_for_group<N, A, G>(
        &self,
        name: N,
        artifact_name: A,
        group_name: G,
        retries: usize,
    ) -> impl Future<Output = reqwest::Result<String>> + Send
    where
        N: AsRef<str> + Send + Sync,
        A: AsRef<str> + Send + Sync,
        G: Display + Send + Sync;

    /// Abort a deployment.
    fn abort(&self, id: Uuid) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Abort all ongoing deployments.
    fn abort_all(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<()>> + Send;

    /// Abort a deployment for a given device.
    fn abort_device(&self, device_id: Uuid) -> impl Future<Output = reqwest::Result<()>> + Send;
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

    async fn create<N, A>(
        &self,
        name: N,
        artifact_name: A,
        devices: &[Uuid],
        retries: usize,
    ) -> reqwest::Result<String>
    where
        N: AsRef<str> + Send + Sync,
        A: AsRef<str> + Send + Sync,
    {
        self.client()
            .post(self.format_url(PATH, None))
            .bearer_auth(self.bearer_token())
            .json(&NewDeployment::new(
                name.as_ref(),
                artifact_name.as_ref(),
                devices,
                retries,
            ))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn create_for_group<N, A, G>(
        &self,
        name: N,
        artifact_name: A,
        group_name: G,
        retries: usize,
    ) -> reqwest::Result<String>
    where
        N: AsRef<str> + Send + Sync,
        A: AsRef<str> + Send + Sync,
        G: Display + Send + Sync,
    {
        self.client()
            .post(self.format_url(format!("{PATH}/group/{group_name}"), None))
            .bearer_auth(self.bearer_token())
            .json(&NewDeployment::new(
                name.as_ref(),
                artifact_name.as_ref(),
                &[],
                retries,
            ))
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

    async fn abort_device(&self, device_id: Uuid) -> reqwest::Result<()> {
        self.client()
            .delete(self.format_url(format!("{PATH}/devices/{device_id}"), None))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .bytes()
            .await
            .map(drop)
    }
}
