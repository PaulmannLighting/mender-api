//!  Implementation of the Mender server API.

pub use devices::Devices;
pub use login::Login;
pub use releases::Releases;
use reqwest::{Certificate, Client, Url};

pub mod devices;
pub mod dto;
mod login;
mod releases;
mod session;

/// Mender server API client.
#[derive(Clone, Debug)]
pub struct Api {
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl Api {
    /// Crate a new API instance.
    pub fn new(base_url: Url, certificate: Option<Certificate>) -> reqwest::Result<Self> {
        let mut builder = Client::builder().use_rustls_tls();

        if let Some(certificate) = certificate {
            builder = builder.add_root_certificate(certificate);
        }

        builder.build().map(|client| Self { base_url, client })
    }
}
