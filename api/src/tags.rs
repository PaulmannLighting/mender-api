use uuid::Uuid;

use crate::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Manage device tags.
pub trait Tags {
    fn list(&self, id: Uuid) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl Tags for Session {
    async fn list(&self, id: Uuid) -> reqwest::Result<String> {
        self.client()
            .get(self.format_url(format!("{PATH}/{id}/tags"), None))
            .bearer_auth(self.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
