use crate::api::dto::ReleaseList;
use crate::api::pager::Pager;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments/releases/list";

/// Releases management API.
pub trait Releases {
    /// List all releases available in the Mender server.
    fn list(&self) -> impl Future<Output = reqwest::Result<ReleaseList>>;
}

impl Releases for Session {
    async fn list(&self) -> reqwest::Result<ReleaseList> {
        Pager::new(self, PATH).iter().await
    }
}
