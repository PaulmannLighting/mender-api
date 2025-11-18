//! Configuration file handling for Mender API client.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use log::error;
use mender_api::{Client, Login, PemCertificate, Session};
use serde::Deserialize;

use crate::ConfigFile;
use crate::config_args::ConfigArgs;

/// Configuration file for Mender API client.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    url: String,
    username: String,
    password: String,
    certificate: PathBuf,
    insecure: bool,
}

impl Config {
    /// Return the URL of the Mender server.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Return the username for authentication.
    #[must_use]
    pub fn username(&self) -> &str {
        &self.username
    }

    /// Return the password for authentication.
    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Return the path to the CA certificate file.
    #[must_use]
    pub fn certificate(&self) -> &Path {
        &self.certificate
    }

    /// Return whether to accept invalid certificates.
    #[must_use]
    pub const fn insecure(&self) -> bool {
        self.insecure
    }

    /// Load configuration from the default path (`$HOME/mender-api.toml`).
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory could not be determined, or if the file could not be read or parsed.
    pub async fn load_or_args(args: impl ConfigArgs) -> Result<Session, ExitCode> {
        let config_file = match ConfigFile::load() {
            Ok(config) => config,
            Err(error) => {
                error!("Failed to load default config file: {error}");
                return Err(ExitCode::FAILURE);
            }
        };

        let certificate = match args
            .certificate()
            .or_else(|| config_file.as_ref().and_then(ConfigFile::certificate))
            .load()
        {
            Ok(cert) => cert,
            Err(code) => return Err(code),
        };

        let Some(url) = args
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
            .map_or_else(|| args.insecure(), ConfigFile::insecure);

        let Ok(client) = Client::new(url, certificate, insecure)
            .inspect_err(|error| error!("Failed to create client: {error}"))
        else {
            return Err(ExitCode::FAILURE);
        };

        let Some(username) = args
            .username()
            .or_else(|| config_file.as_ref().and_then(ConfigFile::username))
        else {
            error!("Username not provided");
            return Err(ExitCode::FAILURE);
        };

        let Some(password) = args
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
