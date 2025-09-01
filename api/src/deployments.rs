use std::fmt::Display;
use std::num::NonZero;

use log::{error, info};
use uuid::Uuid;

use crate::dto::{DeploymentStatus, ListDeployment, NewDeployment, PutDeployment};
use crate::paging::{DEFAULT_PAGE_SIZE, PagedIterator, Pager, Pages};
use crate::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

/// Deployments management API.
pub trait Deployments {
    /// Iterate over deployment pages.
    fn pages(&self, page_size: Option<NonZero<usize>>) -> Pages<'_, '_, ListDeployment>;

    /// List deployment.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PagedIterator<'_, '_, ListDeployment>;

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
    fn pages(&self, page_size: Option<NonZero<usize>>) -> Pages<'_, '_, ListDeployment> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    fn list(&self, page_size: Option<NonZero<usize>>) -> PagedIterator<'_, '_, ListDeployment> {
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
            .json(&PutDeployment::new(DeploymentStatus::Aborted))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn abort_all(&self, page_size: Option<NonZero<usize>>) -> reqwest::Result<()> {
        let mut pages = self.pages(page_size);

        while let Some(page) = pages.next().await {
            let page = page?;
            let mut tasks = Vec::with_capacity(page.len());

            for deployment in page
                .into_iter()
                .filter(|deployment| deployment.status() != DeploymentStatus::Finished)
            {
                let id = deployment.id();
                let this = self.clone();

                tasks.push(tokio::spawn(async move {
                    if let Err(error) = this.abort(id).await {
                        error!("Failed to abort deployment: {error}");
                    } else {
                        info!("Aborted deployment {id}");
                    }
                }));
            }

            for task in tasks {
                task.await.expect("Joining task failed");
            }
        }

        Ok(())
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
