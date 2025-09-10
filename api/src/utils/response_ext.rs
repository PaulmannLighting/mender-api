use log::warn;
use reqwest::Response;

pub trait ResponseExt {
    /// Ensure that the response body is empty, logging a warning if not.
    fn ensure_empty(self) -> impl Future<Output = reqwest::Result<()>> + Send;
}

impl ResponseExt for Response {
    async fn ensure_empty(self) -> reqwest::Result<()> {
        let bytes = self.bytes().await?;

        if !bytes.is_empty() {
            warn!("Expected empty response body, but got some data: {bytes:?}");
        }

        Ok(())
    }
}
