use uuid::Uuid;

use crate::dto::Tag;
use crate::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Manage device tags.
pub trait Tags {
    /// Add a tag to the specified device.
    fn add(
        &self,
        device_id: Uuid,
        tags: &[Tag],
    ) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl Tags for Session {
    async fn add(&self, device_id: Uuid, tags: &[Tag]) -> reqwest::Result<String> {
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
}
