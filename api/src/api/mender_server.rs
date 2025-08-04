//! Extension traits to handle Mender URLs.

use reqwest::{Certificate, Client, Url};

/// Mender server API client.
#[derive(Clone, Debug)]
pub struct MenderServer {
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl MenderServer {
    /// Crate a new `MenderServer` with the specified base URL.
    pub fn new(base_url: Url, certificate: Option<Certificate>) -> reqwest::Result<Self> {
        let mut builder = Client::builder().use_rustls_tls();

        if let Some(certificate) = certificate {
            builder = builder.add_root_certificate(certificate);
        }

        builder.build().map(|client| Self { base_url, client })
    }
}
