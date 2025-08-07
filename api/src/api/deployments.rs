use std::num::NonZero;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::dto::{ListDeployment, NewDeployment};
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

/// Deployments management API.
pub trait Deployments<'this, 'path> {
    /// List deployments.
    fn list(self, page_size: Option<NonZero<usize>>) -> PageIterator<'this, 'path, ListDeployment>;
    /// Collect deployments into a `Vec`.
    fn collect(
        self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<ListDeployment>>> + Send;

    /// Create a new deployment.
    fn create(
        self,
        deployment: &NewDeployment,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl<'session> Deployments<'session, 'session> for &'session Session {
    fn list(
        self,
        page_size: Option<NonZero<usize>>,
    ) -> PageIterator<'session, 'session, ListDeployment> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(
        self,
        page_size: Option<NonZero<usize>>,
    ) -> reqwest::Result<Vec<ListDeployment>> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }

    async fn create(self, deployment: &NewDeployment) -> reqwest::Result<String> {
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
