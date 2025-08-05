use std::num::NonZero;

use uuid::Uuid;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::pager::Pager;
use crate::api::session::Session;

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
}

impl Groups for &Session {
    async fn list(self) -> reqwest::Result<Vec<String>> {
        self.client()
            .get(self.url(PATH))
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
}
