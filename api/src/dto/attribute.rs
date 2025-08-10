use std::fmt::Display;
use std::net::IpAddr;

use chrono::{DateTime, FixedOffset};
use ipnet::{Ipv4Net, Ipv6Net};
use macaddr::MacAddr6;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::dto::scope::Scope;
use crate::dto::status::Status;
use crate::dto::types::OneOrMany;
use crate::dto::{BootloaderIntegration, Country, DeviceType, RootfsType};
use crate::utils::{as_str, display_slice};

/// Available attributes for device in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Attribute {
    /// Known attributes with known types.
    Known(KnownAttribute),
    /// Unknown attributes with unknown types.
    Unknown {
        /// The name of the unknown attribute.
        name: String,
        /// The value of the unknown attribute.
        value: String,
        /// An optional description.
        description: Option<String>,
        /// The scope of this attribute.
        scope: Scope,
    },
}

impl Display for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Known(known) => Display::fmt(known, f),
            Self::Unknown {
                name,
                value,
                description,
                scope,
            } => {
                if let Some(desc) = description {
                    write!(f, "{name}: {value} ({desc}) [{scope}]")
                } else {
                    write!(f, "{name}: {value} [{scope}]")
                }
            }
        }
    }
}

/// Known attributes for device in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(tag = "name")]
pub enum KnownAttribute {
    /// The name of the device.
    #[serde(rename = "name")]
    Name {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The device's MAC address.
    #[serde(rename = "mac")]
    Mac {
        /// The attribute value.
        #[serde(with = "as_str")]
        value: MacAddr6,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The timestamp when the device was created.
    #[serde(rename = "created_ts")]
    Created {
        /// The attribute value.
        value: DateTime<FixedOffset>,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The timestamp when the device was last updated.
    #[serde(rename = "updated_ts")]
    Updated {
        /// The attribute value.
        value: DateTime<FixedOffset>,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The status of the device.
    #[serde(rename = "status")]
    Status {
        /// The attribute value.
        value: Status,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The names of the groups that the device is a member of.
    #[serde(rename = "group")]
    Group {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The name of the artifact currently installed on the device.
    #[serde(rename = "artifact_name")]
    ArtifactName {
        /// The attribute value.
        value: String,

        /// The scope of this attribute.
        scope: Scope,
    },
    /// The model of the CPU on the device.
    #[serde(rename = "cpu_model")]
    CpuModel {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The type of the device.
    #[serde(rename = "device_type")]
    DeviceType {
        /// The attribute value.
        value: DeviceType,

        /// The scope of this attribute.
        scope: Scope,
    },
    /// The hostname of the device.
    #[serde(rename = "hostname")]
    Hostname {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The IPv4 address assigned to the device's first Ethernet interface (eth0).
    #[serde(rename = "ipv4_eth0")]
    Ipv4Eth0 {
        /// The attribute value.
        value: OneOrMany<Ipv4Net>,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The IPv6 address assigned to the device's first Ethernet interface (eth0).
    #[serde(rename = "ipv6_eth0")]
    Ipv6Eth0 {
        /// The attribute value.
        value: OneOrMany<Ipv6Net>,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The kernel running on the device.
    #[serde(rename = "kernel")]
    Kernel {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The MAC address of the device's first Ethernet interface (eth0).
    #[serde(rename = "mac_eth0")]
    MacEth0 {
        /// The attribute value.
        #[serde(with = "as_str")]
        value: MacAddr6,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The amount of memory available on the device, in kilobytes.
    #[serde(rename = "mem_total_kB")]
    MemTotalKB {
        /// The attribute value.
        #[serde(with = "as_str")]
        value: usize,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The bootloader integration used by the device.
    #[serde(rename = "mender_bootloader_integration")]
    MenderBootloaderIntegration {
        /// The attribute value.
        value: BootloaderIntegration,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The Mender client version running on the device.
    #[serde(rename = "mender_client_version")]
    MenderClientVersion {
        /// The attribute value.
        value: Version,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The networking interfaces available on the device.
    #[serde(rename = "network_interfaces")]
    NetworkInterfaces {
        /// The attribute value.
        value: OneOrMany<String>,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The operating system running on the device.
    #[serde(rename = "os")]
    Os {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The file system type of the device's root filesystem.
    #[serde(rename = "rootfs_type")]
    RootfsType {
        /// The attribute value.
        value: RootfsType,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The city where the device is located.
    #[serde(rename = "geo-city")]
    GeoCity {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The country where the device is located.
    #[serde(rename = "geo-country")]
    GeoCountry {
        /// The attribute value.
        value: Country,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The IP address of the device, used for geolocation.
    #[serde(rename = "geo-ip")]
    GeoIp {
        /// The attribute value.
        value: IpAddr,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The timezone of the device, used for geolocation.
    #[serde(rename = "geo-timezone")]
    GeoTimezone {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// Some weird 4-byte MAC-like address. Purpose unknown.
    #[serde(rename = "mac_sit0")]
    MacSit0 {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The checksum of the root filesystem image.
    #[serde(rename = "rootfs-image.checksum")]
    RootfsImageChecksum {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// The version of the root filesystem image installed on the device.
    #[serde(rename = "rootfs-image.version")]
    RootfsImageVersion {
        /// The attribute value.
        value: String,
        /// The scope of this attribute.
        scope: Scope,
    },
    /// Update modules.
    #[serde(rename = "update_modules")]
    UpdateModules {
        /// The attribute value.
        value: Vec<String>,
        /// The scope of this attribute.
        scope: Scope,
    },
}

impl Display for KnownAttribute {
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
            Self::RootfsImageVersion { value, .. } => {
                write!(f, "rootfs-image.version: ")?;
                Display::fmt(value, f)
            }
            Self::UpdateModules { value, .. } => display_slice(value, f),
        }
    }
}
