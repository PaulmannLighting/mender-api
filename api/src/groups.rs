use std::num::NonZero;

use uuid::Uuid;

use crate::dto::PatchGroupResponse;
use crate::pager::{DEFAULT_PAGE_SIZE, Pager};
use crate::session::Session;

const PATH: &str = "/api/management/v1/inventory/groups";

/// Manage groups on the Mender server.
pub trait Groups {
    /// List all groups available in the Mender server.
    fn list(self) -> impl Future<Output = reqwest::Result<Vec<String>>> + Send;

    /// List all devices that are members of the specified group.
    fn devices_of(
        self,
        group_name: &str,
        page_size: Option<NonZero<usize>>,
    ) -> impl Future<Output = reqwest::Result<Vec<Uuid>>> + Send;

    /// Update or create a new group with the specified name and devices.
    fn patch(
        &self,
        name: &str,
        devices: &[Uuid],
    ) -> impl Future<Output = reqwest::Result<PatchGroupResponse>> + Send;
}

impl Groups for &Session {
    async fn list(self) -> reqwest::Result<Vec<String>> {
        self.client()
            .get(self.format_url(PATH, None))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    async fn devices_of(
        self,
        group_name: &str,
        page_size: Option<NonZero<usize>>,
    ) -> reqwest::Result<Vec<Uuid>> {
        Pager::new(
            self,
            &format!("{PATH}/{group_name}/devices"),
            page_size.unwrap_or(DEFAULT_PAGE_SIZE),
        )
        .collect()
        .await
    }

    async fn patch(&self, name: &str, devices: &[Uuid]) -> reqwest::Result<PatchGroupResponse> {
        self.client()
            .patch(self.format_url(format!("{PATH}/{name}/devices"), None))
            .bearer_auth(self.bearer_token())
            .json(devices)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
