use crate::api::dto::DeploymentList;
use crate::api::pager::Pager;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments";

pub trait Deployments {
    /// List deployments.
    fn list(&self) -> impl Future<Output = reqwest::Result<DeploymentList>> + Send;
}

impl Deployments for Session {
    #[allow(async_fn_in_trait)]
    async fn list(&self) -> reqwest::Result<DeploymentList> {
        Pager::new(self, PATH).iter().await
    }
}
