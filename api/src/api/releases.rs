use crate::api::dto::ReleaseList;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments/releases/list";

/// Trait to manage releases in the mender server API.
pub trait Releases {
    /// List all releases available in the Mender server.
    fn list(&self) -> impl Future<Output = reqwest::Result<ReleaseList>>;
}

impl Releases for Session {
    async fn list(&self) -> reqwest::Result<ReleaseList> {
        self.client()
            .get(self.url(PATH))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
