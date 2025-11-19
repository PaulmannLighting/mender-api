use reqwest::{Client, RequestBuilder, Url};

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

    /// Make a GET request.
    pub fn get<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.client
            .get(self.format_url(path, query))
            .bearer_auth(&self.bearer_token)
    }

    /// Make a POST request.
    pub fn post<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.client
            .post(self.format_url(path, query))
            .bearer_auth(&self.bearer_token)
    }

    /// Make a PATCH request.
    pub fn patch<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.client
            .patch(self.format_url(path, query))
            .bearer_auth(&self.bearer_token)
    }

    /// Make a PUT request.
    pub fn put<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.client
            .put(self.format_url(path, query))
            .bearer_auth(&self.bearer_token)
    }

    /// Make a DELETE request.
    pub fn delete<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.client
            .delete(self.format_url(path, query))
            .bearer_auth(&self.bearer_token)
    }

    /// Return the URL to the specified path on the Mender server.
    #[must_use]
    fn format_url<'q, P, Q>(&self, path: P, query: Q) -> Url
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
