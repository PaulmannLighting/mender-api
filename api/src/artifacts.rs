//! Manage artifacts.

use std::num::NonZero;

use crate::dto::Artifact;
use crate::pagination::DEFAULT_PAGE_SIZE;
use crate::{Pager, PaginatedIterator, Session};

const PATH: &str = "api/management/v1/deployments";

/// Artifacts management API.
pub trait Artifacts {
    /// List all artifacts.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Artifact>;
}

impl Artifacts for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Artifact> {
        Pager::new(
            self,
            format!("{PATH}/artifacts/list").into(),
            page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .into()
    }
}
