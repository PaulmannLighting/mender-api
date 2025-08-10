use reqwest::{Client, Url};

use crate::dto::Attribute;

const PATH: &str = "api/devices/v1/inventory";

/// Authorized device API using JSON Web Token (JWT) for authentication.
#[derive(Clone, Debug)]
pub struct Session {
    session: Client,
    base_url: Url,
    jwt: String,
}

impl Session {
    /// Creates a new `Api` instance with the given session and JWT.
    #[must_use]
    pub(crate) const fn new(session: Client, base_url: Url, jwt: String) -> Self {
        Self {
            session,
            base_url,
            jwt,
        }
    }

    /// Return a reference to the underlying `Client`.
    pub(crate) fn client(&self) -> &Client {
        &self.session
    }

    /// Return the URL to the specified path on the Mender server.
    #[must_use]
    pub(crate) fn format_url<P, Q>(&self, path: P, query: Q) -> Url
    where
        P: AsRef<str>,
        Q: Into<Option<String>>,
    {
        let mut url = self.base_url.clone();
        url.set_path(path.as_ref());

        if let Some(query) = query.into() {
            url.set_query(Some(query.as_ref()));
        }

        url
    }

    /// Return a reference to the JWT.
    pub(crate) fn jwt(&self) -> &str {
        &self.jwt
    }
}

impl Session {
    pub async fn update_attributes(&self, attributes: &[Attribute]) -> reqwest::Result<String> {
        self.client()
            .patch(self.format_url(format!("{PATH}/device/attributes"), None))
            .bearer_auth(&self.jwt)
            .json(attributes)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}
