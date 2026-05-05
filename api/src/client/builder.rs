use reqwest::{ClientBuilder, Url};

use crate::Client;

/// Trait to extend the [`ClientBuilder`] with a method to build a [`Client`].
pub trait Builder {
    /// Build the client.
    ///
    /// # Errors
    ///
    /// Returns a [`request::Error`] if building the client fails.
    fn build_with_url(self, base_url: Url) -> reqwest::Result<Client>;
}

impl Builder for ClientBuilder {
    fn build_with_url(self, base_url: Url) -> reqwest::Result<Client> {
        self.build().map(|client| Client { base_url, client })
    }
}
