use crate::api::session::Session;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Manage groups on the Mender server.
pub trait Groups {
    /// List all groups available in the Mender server.
    fn list(&self) -> impl Future<Output = reqwest::Result<Vec<String>>> + Send;
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
}
