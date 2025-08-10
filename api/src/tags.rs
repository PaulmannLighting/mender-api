use crate::device_proxy::DeviceProxy;
use crate::dto::Tag;

const PATH: &str = "/api/management/v1/inventory/devices";

/// Manage device tags.
pub trait Tags {
    /// Add a tag to the specified device.
    fn add(&self, tags: &[Tag]) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl Tags for DeviceProxy<'_> {
    async fn add(&self, tags: &[Tag]) -> reqwest::Result<String> {
        self.session()
            .client()
            .put(
                self.session()
                    .format_url(format!("{PATH}/{}/tags", self.id()), None),
            )
            .bearer_auth(self.session().bearer_token())
            .json(tags)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
