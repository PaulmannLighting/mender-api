use reqwest::{Client, Url};

use crate::{Deployments, Devices, Groups, Releases};

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

/// Crate internals.
impl Session {
    /// Return the URL to the specified path on the Mender server.
    #[must_use]
    pub(crate) fn format_url<P>(&self, path: P) -> Url
    where
        P: AsRef<str>,
    {
        let mut url = self.base_url.clone();
        url.set_path(path.as_ref());
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

/// External API.
impl Session {
    /// Return an opaque type to proxy deployment-related operations.
    #[must_use]
    pub fn deployments(&self) -> impl Deployments {
        self
    }

    /// Return an opaque type to proxy devices-related operations.
    #[must_use]
    pub fn devices(&self) -> impl Devices {
        self
    }

    /// Return an opaque type to proxy groups-related operations.
    #[must_use]
    pub fn groups(&self) -> impl Groups {
        self
    }

    /// Return an opaque type to proxy releases-related operations.
    #[must_use]
    pub fn releases(&self) -> impl Releases {
        self
    }
}
