//!  Implementation of the Mender server API.

use std::num::NonZero;

pub use deployments::Deployments;
pub use devices::Devices;
pub use groups::Groups;
pub use login::Login;
pub use releases::Releases;
use reqwest::{Certificate, Client, Url};

mod deployments;
pub mod devices;
pub mod dto;
mod groups;
mod login;
mod pager;
mod releases;
mod session;

const DEFAULT_PAGE_SIZE: NonZero<usize> =
    NonZero::new(500).expect("Default page should be be non-zero.");

/// Mender server API client.
#[derive(Clone, Debug)]
pub struct Api {
    pub(crate) base_url: Url,
    pub(crate) client: Client,
}

impl Api {
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
        let mut builder = Client::builder().use_rustls_tls();

        if let Some(certificate) = certificate {
            builder = builder
                .add_root_certificate(certificate)
                .danger_accept_invalid_certs(accept_invalid_certificates);
        }

        builder.build().map(|client| Self { base_url, client })
    }
}
