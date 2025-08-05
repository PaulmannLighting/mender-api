use uuid::Uuid;

use crate::api::pager::Pager;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/inventory/groups";

/// Manage groups on the Mender server.
pub trait Groups {
    /// List all groups available in the Mender server.
    fn list(&self) -> impl Future<Output = reqwest::Result<Vec<String>>> + Send;

    /// List all devices that are members of the specified group.
    fn devices_of(
        &self,
        group_name: &str,
    ) -> impl Future<Output = reqwest::Result<Vec<Uuid>>> + Send;
}

impl Groups for Session {
    async fn list(&self) -> reqwest::Result<Vec<String>> {
        self.client()
            .get(self.url(PATH))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    async fn devices_of(&self, group_name: &str) -> reqwest::Result<Vec<Uuid>> {
        Pager::new(self, &format!("{PATH}/{group_name}/devices"))
            .collect()
            .await
    }
}
