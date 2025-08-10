use uuid::Uuid;

use crate::dto::Tag;
use crate::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Manage device tags.
pub trait Tags {
    /// Add tags to the specified device.
    fn add(
        &self,
        device_id: Uuid,
        tags: &[Tag],
    ) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Assign tags to the specified device.
    fn assign(
        &self,
        device_id: Uuid,
        tags: &[Tag],
    ) -> impl Future<Output = reqwest::Result<String>> + Send;

    /// Clear tags of the specified device.
    fn clear(&self, device_id: Uuid) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl Tags for Session {
    async fn add(&self, device_id: Uuid, tags: &[Tag]) -> reqwest::Result<String> {
        self.client()
            .patch(self.format_url(format!("{PATH}/{device_id}/tags"), None))
            .bearer_auth(self.bearer_token())
            .json(tags)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn assign(&self, device_id: Uuid, tags: &[Tag]) -> reqwest::Result<String> {
        self.client()
            .put(self.format_url(format!("{PATH}/{device_id}/tags"), None))
            .bearer_auth(self.bearer_token())
            .json(tags)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }

    async fn clear(&self, device_id: Uuid) -> reqwest::Result<String> {
        self.assign(device_id, &[]).await
    }
}
