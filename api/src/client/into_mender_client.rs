use reqwest::Url;

use crate::Client;

/// Trait to convert a [`reqwest::Client`] into a [`Client`].
pub trait IntoMenderClient {
    /// Convert the [`reqwest::Client`] into a [`Client`] using the given base URL.
    #[must_use]
    fn into_mender_client(self, base_url: Url) -> Client;
}

impl IntoMenderClient for reqwest::Client {
    fn into_mender_client(self, base_url: Url) -> Client {
        Client {
            base_url,
            client: self,
        }
    }
}
