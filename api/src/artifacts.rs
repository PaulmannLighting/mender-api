//! Manage artifacts.

use std::num::NonZero;

use crate::dto::Artifact;
use crate::pagination::DEFAULT_PAGE_SIZE;
use crate::{Pager, PaginatedIterator, Session};

const PATH_V2: &str = "/api/management/v2/deployments";

pub trait Artifacts {
    /// List all artifacts.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Artifact>;
}

impl Artifacts for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PaginatedIterator<'_, '_, Artifact> {
        Pager::new(
            self,
            format!("{PATH_V2}/artifacts").into(),
            page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .into()
    }
}
