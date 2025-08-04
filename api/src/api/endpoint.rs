use std::path::PathBuf;

use reqwest::Url;

use crate::api::MenderServer;

/// Represents an endpoint for the Mender server API.
#[derive(Clone, Debug)]
pub struct Endpoint<'server> {
    pub(crate) server: &'server MenderServer,
    pub(crate) path: PathBuf,
}

impl<'server> Endpoint<'server> {
    /// Create a new `Endpoint` with the specified Mender server.
    pub(crate) fn new(server: &'server MenderServer, path: PathBuf) -> Self {
        Self { server, path }
    }
}

impl Endpoint<'_> {
    /// Return the base URL
    ///
    /// # Panics
    ///
    /// If the path is not valid UTF-8, this will panic.
    #[must_use]
    pub(crate) fn url(&self) -> Url {
        let mut url = self.server.base_url.clone();
        url.set_path(self.path.to_str().expect("Path should be valid UTF-8."));
        url
    }
}
