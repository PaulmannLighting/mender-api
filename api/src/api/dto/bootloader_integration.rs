use std::fmt::Display;

use serde::{Deserialize, Serialize};

/// Available bootloader integrations for devices in the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BootloaderIntegration {
    /// U-boot bootloader integration.
    UBoot,
}

impl Display for BootloaderIntegration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UBoot => write!(f, "uboot"),
        }
    }
}
