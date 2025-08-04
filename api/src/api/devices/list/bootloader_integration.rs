use serde::{Deserialize, Serialize};

/// Available bootloader integrations for devices in the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BootloaderIntegration {
    /// U-boot bootloader integration.
    UBoot,
}
