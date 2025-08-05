use crate::api::dto::{DeploymentList, NewDeployment};
use crate::api::pager::Pager;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

pub trait Deployments {
    /// List deployments.
    fn list(&self) -> impl Future<Output = reqwest::Result<DeploymentList>> + Send;

    /// Create a new deployment.
    fn create(
        &self,
        deployment: &NewDeployment,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl Deployments for Session {
    async fn list(&self) -> reqwest::Result<DeploymentList> {
        Pager::new(self, PATH).iter().await
    }

    async fn create(&self, deployment: &NewDeployment) -> reqwest::Result<String> {
        self.client()
            .post(self.url(PATH))
            .json(deployment)
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
