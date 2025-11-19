//! Manage artifacts.

use std::num::NonZero;

use uuid::Uuid;

use crate::dto::Artifact;
use crate::pagination::DEFAULT_PAGE_SIZE;
use crate::utils::ResponseExt;
use crate::{Pager, PaginatedIterator, Session};

const PATH: &str = "api/management/v1/deployments/artifacts";

/// Artifacts management API.
pub trait Artifacts {
    /// List all artifacts.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Artifact>;

    /// Delete an artifact by its ID.
    fn delete(&self, id: Uuid) -> impl Future<Output = reqwest::Result<()>>;
}

impl Artifacts for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Artifact> {
        Pager::new(
            self,
            format!("{PATH}/list").into(),
            page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .into()
    }

    async fn delete(&self, id: Uuid) -> reqwest::Result<()> {
        self.delete(format!("{PATH}/{id}"), None)
            .send()
            .await?
            .error_for_status()?
            .ensure_empty()
            .await
    }
}
