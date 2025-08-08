use reqwest::{Certificate, Url};

/// Mender server API client.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) base_url: Url,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Crate a new API instance.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if the client could not be built.
    pub fn new(
        base_url: Url,
        certificate: Option<Certificate>,
        accept_invalid_certificates: bool,
    ) -> reqwest::Result<Self> {
        let mut builder = reqwest::Client::builder().use_rustls_tls();

        if let Some(certificate) = certificate {
            builder = builder
                .add_root_certificate(certificate)
                .danger_accept_invalid_certs(accept_invalid_certificates);
        }

        builder.build().map(|client| Self { base_url, client })
    }
}
