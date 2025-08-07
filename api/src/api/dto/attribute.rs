use std::net::IpAddr;

use chrono::{DateTime, FixedOffset};
use ipnet::{Ipv4Net, Ipv6Net};
pub use scope::Scope;
use semver::Version;
use serde::{Deserialize, Serialize};
pub use status::Status;
pub use value::Value;

use crate::api::dto::types::{AsStr, MacAddress, OneOrMany};
use crate::api::dto::{BootloaderIntegration, Country, DeviceType, RootfsType};

mod scope;
mod status;
mod value;

/// Available attributes for devices in the Mender inventory API.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(tag = "name")]
pub enum Attribute {
    /// The name of the device.
    #[serde(rename = "name")]
    Name(Value<String>),
    /// The device's MAC address.
    #[serde(rename = "mac")]
    Mac(Value<MacAddress>),
    /// The timestamp when the device was created.
    #[serde(rename = "created_ts")]
    Created(Value<DateTime<FixedOffset>>),
    /// The timestamp when the device was last updated.
    #[serde(rename = "updated_ts")]
    Updated(Value<DateTime<FixedOffset>>),
    /// The status of the device.
    #[serde(rename = "status")]
    Status(Value<Status>),
    /// The names of the groups that the device is a member of.
    #[serde(rename = "group")]
    Group(Value<String>),
    /// The name of the artifact currently installed on the device.
    #[serde(rename = "artifact_name")]
    ArtifactName(Value<String>),
    /// The model of the CPU on the device.
    #[serde(rename = "cpu_model")]
    CpuModel(Value<String>),
    /// The type of the device.
    #[serde(rename = "device_type")]
    DeviceType(Value<DeviceType>),
    /// The hostname of the device.
    #[serde(rename = "hostname")]
    Hostname(Value<String>),
    /// The IPv4 address assigned to the device's first Ethernet interface (eth0).
    #[serde(rename = "ipv4_eth0")]
    Ipv4Eth0(Value<OneOrMany<Ipv4Net>>),
    /// The IPv6 address assigned to the device's first Ethernet interface (eth0).
    #[serde(rename = "ipv6_eth0")]
    Ipv6Eth0(Value<OneOrMany<Ipv6Net>>),
    /// The kernel running on the device.
    #[serde(rename = "kernel")]
    Kernel(Value<String>),
    /// The MAC address of the device's first Ethernet interface (eth0).
    #[serde(rename = "mac_eth0")]
    MacEth0(Value<MacAddress>),
    /// The amount of memory available on the device, in kilobytes.
    #[serde(rename = "mem_total_kB")]
    MemTotalKB(Value<AsStr<u32>>),
    /// The bootloader integration used by the device.
    #[serde(rename = "mender_bootloader_integration")]
    MenderBootloaderIntegration(Value<BootloaderIntegration>),
    /// The Mender client version running on the device.
    #[serde(rename = "mender_client_version")]
    MenderClientVersion(Value<Version>),
    /// The networking interfaces available on the device.
    #[serde(rename = "network_interfaces")]
    NetworkInterfaces(Value<OneOrMany<String>>),
    /// The operating system running on the device.
    #[serde(rename = "os")]
    Os(Value<String>),
    /// The file system type of the device's root filesystem.
    #[serde(rename = "rootfs_type")]
    RootfsType(Value<RootfsType>),
    /// The city where the device is located.
    #[serde(rename = "geo-city")]
    GeoCity(Value<String>),
    /// The country where the device is located.
    #[serde(rename = "geo-country")]
    GeoCountry(Value<Country>),
    /// The IP address of the device, used for geolocation.
    #[serde(rename = "geo-ip")]
    GeoIp(Value<IpAddr>),
    /// The timezone of the device, used for geolocation.
    #[serde(rename = "geo-timezone")]
    GeoTimezone(Value<String>),
    /// Some weird 4-byte MAC-like address. Purpose unknown.
    #[serde(rename = "mac_sit0")]
    MacSit0(Value<String>),
    /// The checksum of the root filesystem image.
    #[serde(rename = "rootfs-image.checksum")]
    RootfsImageChecksum(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.deb.mender_update_module")]
    RootfsImageUpdateModuleDebMenderUpdateModule(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.deb.version")]
    RootfsImageUpdateModuleDebVersion(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.directory.mender_update_module")]
    RootfsImageUpdateModuleDirectoryMenderUpdateModule(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.directory.version")]
    RootfsImageUpdateModuleDirectoryVersion(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.docker.mender_update_module")]
    RootfsImageUpdateModuleDockerMenderUpdateModule(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.docker.version")]
    RootfsImageUpdateModuleDockerVersion(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.rpm.mender_update_module")]
    RootfsImageUpdateModuleRpmMenderUpdateModule(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.rpm.version")]
    RootfsImageUpdateModuleRpmVersion(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.script.mender_update_module")]
    RootfsImageUpdateModuleScriptMenderUpdateModule(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.script.version")]
    RootfsImageUpdateModuleScriptVersion(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.single-file.mender_update_module")]
    RootfsImageUpdateModuleSingleFileMenderUpdateModule(Value<String>),
    /// TODO: Field with unknown purpose.
    #[serde(rename = "rootfs-image.update-module.single-file.version")]
    RootfsImageUpdateModuleSingleFileVersion(Value<String>),
    /// The version of the root filesystem image installed on the device.
    #[serde(rename = "rootfs-image.version")]
    RootfsImageVersion(Value<String>),
    /// Update modules.
    #[serde(rename = "update_modules")]
    UpdateModules(Value<Vec<String>>),
}
