use reqwest::{ClientBuilder, Url};

pub use self::builder::Builder;

mod builder;

/// Mender server API client.
#[derive(Clone, Debug)]
pub struct Client {
    pub(crate) base_url: Url,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Crate a new mender client.
    #[must_use]
    pub fn new(base_url: Url) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    /// Return a new client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }
}
