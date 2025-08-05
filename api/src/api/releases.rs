use crate::api::dto::Release;
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments/releases/list";

/// Releases management API.
pub trait Releases {
    /// List all releases available in the Mender server.
    fn list(&self) -> PageIterator<Release>;
}

impl Releases for Session {
    fn list(&self) -> PageIterator<Release> {
        Pager::new(self, PATH).into()
    }
}
