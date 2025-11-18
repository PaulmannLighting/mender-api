use std::path::Path;
use std::process::ExitCode;

use log::error;
use mender_api::{Client, Login, PemCertificate, Session};

use crate::ConfigFile;

/// Trait for accessing configuration arguments.
pub trait ConfigArgs {
    /// Mender server URL.
    #[must_use]
    fn url(&self) -> Option<&str>;

    /// Path to the client certificate.
    #[must_use]
    fn certificate(&self) -> Option<&Path>;

    /// Username for authentication.
    #[must_use]
    fn username(&self) -> Option<&str>;

    /// Password for authentication.
    #[must_use]
    fn password(&self) -> Option<&str>;

    /// Whether to skip TLS verification.
    #[must_use]
    fn insecure(&self) -> bool;

    /// Load configuration from the default path (`$HOME/mender-api.toml`).
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory could not be determined, or if the file could not be read or parsed.
    #[expect(async_fn_in_trait)]
    async fn login(&self) -> Result<Session, ExitCode> {
        let config_file = match ConfigFile::load() {
            Ok(config) => config,
            Err(error) => {
                error!("Failed to load default config file: {error}");
                return Err(ExitCode::FAILURE);
            }
        };

        let certificate = match self
            .certificate()
            .or_else(|| config_file.as_ref().and_then(ConfigFile::certificate))
            .load()
        {
            Ok(cert) => cert,
            Err(code) => return Err(code),
        };

        let Some(url) = self
            .url()
            .or_else(|| config_file.as_ref().and_then(ConfigFile::url))
        else {
            error!("Mender server URL not provided");
            return Err(ExitCode::FAILURE);
        };

        let Ok(url) = url
            .parse()
            .inspect_err(|error| error!("Invalid URL: {error}"))
        else {
            return Err(ExitCode::FAILURE);
        };

        let insecure = config_file
            .as_ref()
            .map_or_else(|| self.insecure(), ConfigFile::insecure);

        let Ok(client) = Client::new(url, certificate, insecure)
            .inspect_err(|error| error!("Failed to create client: {error}"))
        else {
            return Err(ExitCode::FAILURE);
        };

        let Some(username) = self
            .username()
            .or_else(|| config_file.as_ref().and_then(ConfigFile::username))
        else {
            error!("Username not provided");
            return Err(ExitCode::FAILURE);
        };

        let Some(password) = self
            .password()
            .or_else(|| config_file.as_ref().and_then(ConfigFile::password))
        else {
            error!("Password not provided");
            return Err(ExitCode::FAILURE);
        };

        client.login(username, password).await.map_err(|error| {
            error!("Login failed: {error}");
            ExitCode::FAILURE
        })
    }
}
