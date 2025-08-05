use crate::api::dto::{ListDeployment, NewDeployment};
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

/// Deployments management API.
pub trait Deployments {
    /// List deployments.
    fn iter(&self) -> PageIterator<ListDeployment>;

    /// Create a new deployment.
    fn create(
        &self,
        deployment: &NewDeployment,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Collect deployments into a `Vec`.
    fn collect(&self) -> impl Future<Output = reqwest::Result<Vec<ListDeployment>>> + Send;
}

impl Deployments for Session {
    fn iter(&self) -> PageIterator<ListDeployment> {
        Pager::new(self, PATH).into()
    }

    async fn create(&self, deployment: &NewDeployment) -> reqwest::Result<String> {
        self.client()
            .post(self.url(PATH))
            .bearer_auth(self.bearer_token())
            .json(deployment)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn collect(&self) -> reqwest::Result<Vec<ListDeployment>> {
        Pager::new(self, PATH).collect().await
    }
}
