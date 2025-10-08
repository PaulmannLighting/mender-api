use std::num::NonZero;

use uuid::Uuid;

use crate::PaginatedIterator;
use crate::dto::PatchGroupResponse;
use crate::pagination::{DEFAULT_PAGE_SIZE, Pager};
use crate::session::Session;

const PATH: &str = "/api/management/v1/inventory/groups";

/// Manage groups on the Mender server.
pub trait Groups {
    /// List all groups available in the Mender server.
    fn list(&self) -> impl Future<Output = reqwest::Result<Vec<String>>> + Send;

    /// List all device that are members of the specified group.
    fn devices_of(
        &self,
        group_name: &str,
        page_size: Option<NonZero<usize>>,
    ) -> PaginatedIterator<'_, '_, Uuid>;

    /// Update or create a new group with the specified name and device.
    fn patch(
        &self,
        name: &str,
        devices: &[Uuid],
    ) -> impl Future<Output = reqwest::Result<PatchGroupResponse>> + Send;
}

impl Groups for Session {
    async fn list(&self) -> reqwest::Result<Vec<String>> {
        self.client()
            .get(self.format_url(PATH, None))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    fn devices_of(
        &self,
        group_name: &str,
        page_size: Option<NonZero<usize>>,
    ) -> PaginatedIterator<'_, '_, Uuid> {
        Pager::new(
            self,
            format!("{PATH}/{group_name}/devices").into(),
            page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .into()
    }

    async fn patch(&self, name: &str, devices: &[Uuid]) -> reqwest::Result<PatchGroupResponse> {
        self.client()
            .patch(self.format_url(format!("{PATH}/{name}/device"), None))
            .bearer_auth(self.bearer_token())
            .json(devices)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
