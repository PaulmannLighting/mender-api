use reqwest::{Client, Url};

/// Represents an endpoint for the Mender server API.
#[derive(Clone, Debug)]
pub struct Session {
    client: Client,
    base_url: Url,
    bearer_token: String,
}

impl Session {
    /// Create a new `Endpoint` with the specified Mender server.
    pub(crate) fn new(client: Client, base_url: Url, bearer_token: String) -> Self {
        Self {
            client,
            base_url,
            bearer_token,
        }
    }
}

impl Session {
    /// Return the URL to the specified path on the Mender server.
    #[must_use]
    pub(crate) fn url(&self, path: &str) -> Url {
        let mut url = self.base_url.clone();
        url.set_path(path);
        url
    }

    /// Return request client for the Mender server.
    #[must_use]
    pub(crate) fn client(&self) -> &Client {
        &self.client
    }

    /// Return the bearer token for authentication.
    #[must_use]
    pub(crate) fn bearer_token(&self) -> &str {
        &self.bearer_token
    }
}
