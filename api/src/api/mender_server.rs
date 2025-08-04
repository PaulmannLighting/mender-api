//! Extension traits to handle Mender URLs.

use reqwest::header::HeaderMap;
use reqwest::{Certificate, Client, Url};

use crate::api::api_version::ApiVersion;
use crate::api::endpoint::Endpoint;
use crate::api::management::Management;

/// Mender server API client.
#[derive(Clone, Debug)]
pub struct MenderServer {
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl MenderServer {
    /// Crate a new `MenderServer` with the specified base URL.
    pub fn new(base_url: Url, certificate: Option<Certificate>) -> reqwest::Result<Self> {
        let mut builder = Client::builder();

        if let Some(certificate) = certificate {
            builder = builder.add_root_certificate(certificate);
        }

        builder.build().map(|client| Self { base_url, client })
    }

    /// Return the mender management API.
    #[must_use]
    pub fn management(&self, version: ApiVersion) -> impl Management {
        Endpoint::new(self, format!("/api/management/{version}").into())
    }
}
