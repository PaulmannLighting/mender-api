use std::num::NonZero;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::dto::Release;
use crate::api::pager::{PageIterator, Pager};
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/deployments/deployments/releases/list";

/// Releases management API.
pub trait Releases<'this, 'path> {
    /// List all releases available in the Mender server.
    fn list(self, page_size: Option<NonZero<usize>>) -> PageIterator<'this, 'path, Release>;

    /// Collect releases into a `Vec`.
    fn collect(
        self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<Release>>> + Send;
}

impl<'session> Releases<'session, 'session> for &'session Session {
    fn list(self, page_size: Option<NonZero<usize>>) -> PageIterator<'session, 'session, Release> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(self, page_size: Option<NonZero<usize>>) -> reqwest::Result<Vec<Release>> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }
}
