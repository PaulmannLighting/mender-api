use std::fmt::Display;
use std::net::IpAddr;

use chrono::{DateTime, FixedOffset};
use ipnet::{Ipv4Net, Ipv6Net};
use macaddr::MacAddr6;
pub use scope::Scope;
use semver::Version;
use serde::{Deserialize, Serialize};
pub use status::Status;

use crate::dto::types::OneOrMany;
use crate::dto::{BootloaderIntegration, Country, DeviceType, RootfsType};
use crate::utils::{as_str, display_slice};

mod scope;
mod status;

/// Available attributes for devices in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(tag = "name")]
pub enum Attribute {
    /// The name of the device.
    #[serde(rename = "name")]
    Name { value: String, scope: Scope },
    /// The device's MAC address.
    #[serde(rename = "mac")]
    Mac {
        #[serde(with = "as_str")]
        value: MacAddr6,
        scope: Scope,
    },
    /// The timestamp when the device was created.
    #[serde(rename = "created_ts")]
    Created {
        value: DateTime<FixedOffset>,
        scope: Scope,
    },
    /// The timestamp when the device was last updated.
    #[serde(rename = "updated_ts")]
    Updated {
        value: DateTime<FixedOffset>,
        scope: Scope,
    },
    /// The status of the device.
    #[serde(rename = "status")]
    Status { value: Status, scope: Scope },
    /// The names of the groups that the device is a member of.
    #[serde(rename = "group")]
    Group { value: String, scope: Scope },
    /// The name of the artifact currently installed on the device.
    #[serde(rename = "artifact_name")]
    ArtifactName { value: String, scope: Scope },
    /// The model of the CPU on the device.
    #[serde(rename = "cpu_model")]
    CpuModel { value: String, scope: Scope },
    /// The type of the device.
    #[serde(rename = "device_type")]
    DeviceType { value: DeviceType, scope: Scope },
    /// The hostname of the device.
    #[serde(rename = "hostname")]
    Hostname { value: String, scope: Scope },
    /// The IPv4 address assigned to the device's first Ethernet interface (eth0).
    #[serde(rename = "ipv4_eth0")]
    Ipv4Eth0 {
        value: OneOrMany<Ipv4Net>,
        scope: Scope,
    },
    /// The IPv6 address assigned to the device's first Ethernet interface (eth0).
    #[serde(rename = "ipv6_eth0")]
    Ipv6Eth0 {
        value: OneOrMany<Ipv6Net>,
        scope: Scope,
    },
    /// The kernel running on the device.
    #[serde(rename = "kernel")]
    Kernel { value: String, scope: Scope },
    /// The MAC address of the device's first Ethernet interface (eth0).
    #[serde(rename = "mac_eth0")]
    MacEth0 {
        #[serde(with = "as_str")]
        value: MacAddr6,
        scope: Scope,
    },
    /// The amount of memory available on the device, in kilobytes.
    #[serde(rename = "mem_total_kB")]
    MemTotalKB {
        #[serde(with = "as_str")]
        value: u32,
        scope: Scope,
    },
    /// The bootloader integration used by the device.
    #[serde(rename = "mender_bootloader_integration")]
    MenderBootloaderIntegration {
        value: BootloaderIntegration,
        scope: Scope,
    },
    /// The Mender client version running on the device.
    #[serde(rename = "mender_client_version")]
    MenderClientVersion { value: Version, scope: Scope },
    /// The networking interfaces available on the device.
    #[serde(rename = "network_interfaces")]
    NetworkInterfaces {
        value: OneOrMany<String>,
        scope: Scope,
    },
    /// The operating system running on the device.
    #[serde(rename = "os")]
    Os { value: String, scope: Scope },
    /// The file system type of the device's root filesystem.
    #[serde(rename = "rootfs_type")]
    RootfsType { value: RootfsType, scope: Scope },
    /// The city where the device is located.
    #[serde(rename = "geo-city")]
    GeoCity { value: String, scope: Scope },
    /// The country where the device is located.
    #[serde(rename = "geo-country")]
    GeoCountry { value: Country, scope: Scope },
    /// The IP address of the device, used for geolocation.
    #[serde(rename = "geo-ip")]
    GeoIp { value: IpAddr, scope: Scope },
    /// The timezone of the device, used for geolocation.
    #[serde(rename = "geo-timezone")]
    GeoTimezone { value: String, scope: Scope },
    /// Some weird 4-byte MAC-like address. Purpose unknown.
    #[serde(rename = "mac_sit0")]
    MacSit0 { value: String, scope: Scope },
    /// The checksum of the root filesystem image.
    #[serde(rename = "rootfs-image.checksum")]
    RootfsImageChecksum { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.deb.mender_update_module")]
    RootfsImageUpdateModuleDebMenderUpdateModule { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.deb.version")]
    RootfsImageUpdateModuleDebVersion { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.directory.mender_update_module")]
    RootfsImageUpdateModuleDirectoryMenderUpdateModule { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.directory.version")]
    RootfsImageUpdateModuleDirectoryVersion { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.docker.mender_update_module")]
    RootfsImageUpdateModuleDockerMenderUpdateModule { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.docker.version")]
    RootfsImageUpdateModuleDockerVersion { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.rpm.mender_update_module")]
    RootfsImageUpdateModuleRpmMenderUpdateModule { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.rpm.version")]
    RootfsImageUpdateModuleRpmVersion { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.script.mender_update_module")]
    RootfsImageUpdateModuleScriptMenderUpdateModule { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.script.version")]
    RootfsImageUpdateModuleScriptVersion { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.single-file.mender_update_module")]
    RootfsImageUpdateModuleSingleFileMenderUpdateModule { value: String, scope: Scope },
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.single-file.version")]
    RootfsImageUpdateModuleSingleFileVersion { value: String, scope: Scope },
    /// The version of the root filesystem image installed on the device.
    #[serde(rename = "rootfs-image.version")]
    RootfsImageVersion { value: String, scope: Scope },
    /// Update modules.
    #[serde(rename = "update_modules")]
    UpdateModules { value: Vec<String>, scope: Scope },
}

impl Display for Attribute {
    #[allow(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Name { value, .. } => {
                write!(f, "name: ")?;
                Display::fmt(value, f)
            }
            Self::Mac { value, .. } => {
                write!(f, "mac: ")?;
                Display::fmt(value, f)
            }
            Self::Created { value, .. } => {
                write!(f, "created_ts: ")?;
                Display::fmt(value, f)
            }
            Self::Updated { value, .. } => {
                write!(f, "updated_ts: ")?;
                Display::fmt(value, f)
            }
            Self::Status { value, .. } => {
                write!(f, "status: ")?;
                Display::fmt(value, f)
            }
            Self::Group { value, .. } => {
                write!(f, "group: ")?;
                Display::fmt(value, f)
            }
            Self::ArtifactName { value, .. } => {
                write!(f, "artifact_name: ")?;
                Display::fmt(value, f)
            }
            Self::CpuModel { value, .. } => {
                write!(f, "cpu_model: ")?;
                Display::fmt(value, f)
            }
            Self::DeviceType { value, .. } => {
                write!(f, "device_type: ")?;
                Display::fmt(value, f)
            }
            Self::Hostname { value, .. } => {
                write!(f, "hostname: ")?;
                Display::fmt(value, f)
            }
            Self::Ipv4Eth0 { value, .. } => {
                write!(f, "ipv4_eth0: ")?;
                Display::fmt(value, f)
            }
            Self::Ipv6Eth0 { value, .. } => {
                write!(f, "ipv6_eth0: ")?;
                Display::fmt(value, f)
            }
            Self::Kernel { value, .. } => {
                write!(f, "kernel: ")?;
                Display::fmt(value, f)
            }
            Self::MacEth0 { value, .. } => {
                write!(f, "mac_eth0: ")?;
                Display::fmt(value, f)
            }
            Self::MemTotalKB { value, .. } => {
                write!(f, "mem_total_kB: ")?;
                Display::fmt(value, f)
            }
            Self::MenderBootloaderIntegration { value, .. } => {
                write!(f, "mender_bootloader_integration: ")?;
                Display::fmt(value, f)
            }
            Self::MenderClientVersion { value, .. } => {
                write!(f, "mender_client_version: ")?;
                Display::fmt(value, f)
            }
            Self::NetworkInterfaces { value, .. } => {
                write!(f, "network_interfaces: ")?;
                Display::fmt(value, f)
            }
            Self::Os { value, .. } => {
                write!(f, "os: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsType { value, .. } => {
                write!(f, "rootfs_type: ")?;
                Display::fmt(value, f)
            }
            Self::GeoCity { value, .. } => {
                write!(f, "geo-city: ")?;
                Display::fmt(value, f)
            }
            Self::GeoCountry { value, .. } => {
                write!(f, "geo-country: ")?;
                Display::fmt(value, f)
            }
            Self::GeoIp { value, .. } => {
                write!(f, "geo-ip: ")?;
                Display::fmt(value, f)
            }
            Self::GeoTimezone { value, .. } => {
                write!(f, "geo-timezone: ")?;
                Display::fmt(value, f)
            }
            Self::MacSit0 { value, .. } => {
                write!(f, "mac_sit0: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageChecksum { value, .. } => {
                write!(f, "rootfs-image.checksum: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleDebMenderUpdateModule { value, .. } => {
                write!(f, "rootfs-image.update-module.deb.mender_update_module: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleDebVersion { value, .. } => {
                write!(f, "rootfs-image.update-module.deb.version: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleDirectoryMenderUpdateModule { value, .. } => {
                write!(
                    f,
                    "rootfs-image.update-module.directory.mender_update_module: "
                )?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleDirectoryVersion { value, .. } => {
                write!(f, "rootfs-image.update-module.directory.version: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleDockerMenderUpdateModule { value, .. } => {
                write!(
                    f,
                    "rootfs-image.update-module.docker.mender_update_module: "
                )?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleDockerVersion { value, .. } => {
                write!(f, "rootfs-image.update-module.docker.version: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleRpmMenderUpdateModule { value, .. } => {
                write!(f, "rootfs-image.update-module.rpm.mender_update_module: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleRpmVersion { value, .. } => {
                write!(f, "rootfs-image.update-module.rpm.version: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleScriptMenderUpdateModule { value, .. } => {
                write!(
                    f,
                    "rootfs-image.update-module.script.mender_update_module: "
                )?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleScriptVersion { value, .. } => {
                write!(f, "rootfs-image.update-module.script.version: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleSingleFileMenderUpdateModule { value, .. } => {
                write!(
                    f,
                    "rootfs-image.update-module.single-file.mender_update_module: "
                )?;
                Display::fmt(value, f)
            }
            Self::RootfsImageUpdateModuleSingleFileVersion { value, .. } => {
                write!(f, "rootfs-image.update-module.single-file.version: ")?;
                Display::fmt(value, f)
            }
            Self::RootfsImageVersion { value, .. } => {
                write!(f, "rootfs-image.version: ")?;
                Display::fmt(value, f)
            }
            Self::UpdateModules { value, .. } => display_slice(value, f),
        }
    }
}
