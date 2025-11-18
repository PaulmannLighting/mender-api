use reqwest::{Client, Url};

/// A session on the Mender server API.
#[derive(Clone, Debug)]
pub struct Session {
    client: Client,
    base_url: Url,
    bearer_token: String,
}

impl Session {
    /// Create a new `Endpoint` with the specified Mender server.
    pub(crate) const fn new(client: Client, base_url: Url, bearer_token: String) -> Self {
        Self {
            client,
            base_url,
            bearer_token,
        }
    }

    /// Return request client for the Mender server.
    #[must_use]
    pub(crate) const fn client(&self) -> &Client {
        &self.client
    }

    /// Return the bearer token for authentication.
    #[must_use]
    pub(crate) fn bearer_token(&self) -> &str {
        &self.bearer_token
    }

    /// Return the URL to the specified path on the Mender server.
    #[must_use]
    pub(crate) fn format_url<'q, P, Q>(&self, path: P, query: Q) -> Url
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        let mut url = self.base_url.clone();
        url.set_path(path.as_ref());

        if let Some(query) = query.into() {
            url.set_query(Some(query));
        }

        url
    }
}
