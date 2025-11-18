//! Configuration file handling for Mender API client.

use std::env::home_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

use log::warn;
use serde::Deserialize;

use crate::config_args::ConfigArgs;

const DEFAULT_FILE_NAME: &str = "mender-api.toml";

/// Configuration file for Mender API client.
#[derive(Clone, Debug, Deserialize)]
pub struct ConfigFile {
    url: Option<String>,
    certificate: Option<PathBuf>,
    username: Option<String>,
    password: Option<String>,
    #[serde(default)]
    insecure: bool,
}

impl ConfigFile {
    /// Load configuration from the default path (`$HOME/mender-api.toml`).
    ///
    /// # Errors
    ///
    /// Returns an error if the home directory could not be determined, or if the file could not be read or parsed.
    pub fn load() -> Result<Option<Self>, Box<dyn std::error::Error>> {
        let Some(config) = read_to_string(
            home_dir()
                .ok_or("Could not determine home directory")?
                .join(DEFAULT_FILE_NAME),
        )
        .ok() else {
            warn!("No config file found at default path, continuing without it.");
            return Ok(None);
        };

        Ok(Some(toml::from_str(&config)?))
    }
}

impl ConfigArgs for ConfigFile {
    fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    fn certificate(&self) -> Option<&Path> {
        self.certificate.as_deref()
    }

    fn username(&self) -> Option<&str> {
        self.username.as_deref()
    }

    fn password(&self) -> Option<&str> {
        self.password.as_deref()
    }

    fn insecure(&self) -> bool {
        self.insecure
    }
}
