use crate::api::dto::Release;
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments/releases/list";

/// Releases management API.
pub trait Releases<'a> {
    /// List all releases available in the Mender server.
    fn list(self) -> PageIterator<'a, 'static, Release>;

    /// Collect releases into a `Vec`.
    fn collect(self) -> impl Future<Output = reqwest::Result<Vec<Release>>> + Send;
}

impl<'session> Releases<'session> for &'session Session {
    fn list(self) -> PageIterator<'session, 'static, Release> {
        Pager::new(self, PATH).into()
    }

    async fn collect(self) -> reqwest::Result<Vec<Release>> {
        Pager::new(self, PATH).collect().await
    }
}
