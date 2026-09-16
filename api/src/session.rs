use reqwest::{Client, Method, RequestBuilder, Url};

/// A session on the Mender server API.
#[derive(Clone, Debug)]
pub struct Session {
    client: Client,
    base_url: Url,
    bearer_token: String,
}

impl Session {
    /// Create an authenticated session for the specified Mender server.
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
        self.request(Method::GET, path, query)
    }

    /// Make a POST request.
    pub fn post<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.request(Method::POST, path, query)
    }

    /// Make a PATCH request.
    pub fn patch<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.request(Method::PATCH, path, query)
    }

    /// Make a PUT request.
    pub fn put<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.request(Method::PUT, path, query)
    }

    /// Make a DELETE request.
    pub fn delete<'q, P, Q>(&self, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.request(Method::DELETE, path, query)
    }

    /// Build a request with the session's base URL and bearer token.
    fn request<'q, P, Q>(&self, method: Method, path: P, query: Q) -> RequestBuilder
    where
        P: AsRef<str>,
        Q: Into<Option<&'q str>>,
    {
        self.client
            .request(method, self.format_url(path, query))
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

#[cfg(test)]
mod tests {
    use reqwest::header::AUTHORIZATION;
    use reqwest::{Client, Method, Url};

    use super::Session;

    const BASE_URL: &str = "https://mender.example.com/base?existing=value";
    const DEVICE_PATH: &str = "/api/management/v1/inventory/devices";
    const QUERY: &str = "per_page=10&page=2";
    const TOKEN: &str = "test-token";

    #[test]
    fn requests_preserve_method_url_and_authentication() {
        let session = Session::new(Client::new(), Url::parse(BASE_URL).unwrap(), TOKEN.into());
        let requests = [
            (Method::GET, session.get(DEVICE_PATH, QUERY)),
            (Method::POST, session.post(DEVICE_PATH, QUERY)),
            (Method::PATCH, session.patch(DEVICE_PATH, QUERY)),
            (Method::PUT, session.put(DEVICE_PATH, QUERY)),
            (Method::DELETE, session.delete(DEVICE_PATH, QUERY)),
        ];

        for (method, builder) in requests {
            let request = builder.build().unwrap();
            assert_eq!(request.method(), method);
            assert_eq!(request.url().scheme(), "https");
            assert_eq!(request.url().host_str(), Some("mender.example.com"));
            assert_eq!(request.url().path(), DEVICE_PATH);
            assert_eq!(request.url().query(), Some(QUERY));
            assert_eq!(request.headers()[AUTHORIZATION], "Bearer test-token");
        }

        let request = session.get(DEVICE_PATH, None).build().unwrap();
        assert_eq!(request.url().query(), Some("existing=value"));
    }
}
