use std::num::NonZero;

use crate::dto::Release;
use crate::pagination::{DEFAULT_PAGE_SIZE, Pager, PaginatedIterator};
use crate::session::Session;

const PATH: &str = "/api/management/v1/deployment/deployment/releases/list";

/// Releases management API.
pub trait Releases {
    /// List all releases available in the Mender server.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Release>;

    /// Collect releases into a `Vec`.
    fn collect(
        &self,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<Release>>> + Send;
}

impl Releases for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Release> {
        Pager::new(self, PATH.into(), page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn collect(&self, page_size: Option<NonZero<usize>>) -> reqwest::Result<Vec<Release>> {
        Pager::new(self, PATH.into(), page_size.unwrap_or(DEFAULT_PAGE_SIZE))
            .collect()
            .await
    }
}
