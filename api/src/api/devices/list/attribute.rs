use std::net::IpAddr;

use chrono::{DateTime, FixedOffset};
use ipnet::{Ipv4Net, Ipv6Net};
use semver::Version;
use serde::{Deserialize, Serialize};

use super::mac_address::MacAddress;
use super::value::Value;
use crate::api::devices::list::as_str::AsStr;
use crate::api::devices::list::one_or_many::OneOrMany;

#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(tag = "name")]
pub enum Attribute {
    #[serde(rename = "mac")]
    Mac(Value<MacAddress>),
    #[serde(rename = "created_ts")]
    Created(Value<DateTime<FixedOffset>>),
    #[serde(rename = "updated_ts")]
    Updated(Value<DateTime<FixedOffset>>),
    #[serde(rename = "status")]
    Status(Value<String>), // TODO: use enum for status
    #[serde(rename = "group")]
    Group(Value<String>),
    #[serde(rename = "artifact_name")]
    ArtifactName(Value<String>),
    #[serde(rename = "cpu_model")]
    CpuModel(Value<String>),
    #[serde(rename = "device_type")]
    DeviceType(Value<String>),
    #[serde(rename = "hostname")]
    Hostname(Value<String>),
    #[serde(rename = "ipv4_eth0")]
    Ipv4Eth0(Value<Ipv4Net>),
    #[serde(rename = "ipv6_eth0")]
    Ipv6Eth0(Value<OneOrMany<Ipv6Net>>),
    #[serde(rename = "kernel")]
    Kernel(Value<String>),
    #[serde(rename = "mac_eth0")]
    MacEth0(Value<MacAddress>),
    #[serde(rename = "mem_total_kB")]
    MemTotalKB(Value<AsStr<u32>>),
    #[serde(rename = "mender_bootloader_integration")]
    MenderBootloaderIntegration(Value<String>), // TODO: use enum for bootloader integration
    #[serde(rename = "mender_client_version")]
    MenderClientVersion(Value<Version>),
    #[serde(rename = "network_interfaces")]
    NetworkInterfaces(Value<OneOrMany<String>>),
    #[serde(rename = "os")]
    Os(Value<String>),
    #[serde(rename = "rootfs_type")]
    RootfsType(Value<String>), // TODO: use enum for rootfs type
    #[serde(rename = "geo-city")]
    GeoCity(Value<String>),
    #[serde(rename = "geo-country")]
    GeoCountry(Value<String>),
    #[serde(rename = "geo-ip")]
    GeoIp(Value<IpAddr>),
    #[serde(rename = "geo-timezone")]
    GeoTimezone(Value<String>),
    #[serde(rename = "mac_sit0")]
    MacSit0(Value<String>), // This is a 4-byte MAC address
    #[serde(rename = "rootfs-image.checksum")]
    RootfsImageChecksum(Value<String>),
    #[serde(rename = "rootfs-image.update-module.deb.mender_update_module")]
    RootfsImageUpdateModuleDebMenderUpdateModule(Value<String>),
    #[serde(rename = "rootfs-image.update-module.deb.version")]
    RootfsImageUpdateModuleDebVersion(Value<String>),
    #[serde(rename = "rootfs-image.update-module.directory.mender_update_module")]
    RootfsImageUpdateModuleDirectoryMenderUpdateModule(Value<String>),
    #[serde(rename = "rootfs-image.update-module.directory.version")]
    RootfsImageUpdateModuleDirectoryVersion(Value<String>),
    #[serde(rename = "rootfs-image.update-module.docker.mender_update_module")]
    RootfsImageUpdateModuleDockerMenderUpdateModule(Value<String>),
    #[serde(rename = "rootfs-image.update-module.docker.version")]
    RootfsImageUpdateModuleDockerVersion(Value<String>),
    #[serde(rename = "rootfs-image.update-module.rpm.mender_update_module")]
    RootfsImageUpdateModuleRpmMenderUpdateModule(Value<String>),
    #[serde(rename = "rootfs-image.update-module.rpm.version")]
    RootfsImageUpdateModuleRpmVersion(Value<String>),
    #[serde(rename = "rootfs-image.update-module.script.mender_update_module")]
    RootfsImageUpdateModuleScriptMenderUpdateModule(Value<String>),
    #[serde(rename = "rootfs-image.update-module.script.version")]
    RootfsImageUpdateModuleScriptVersion(Value<String>),
    #[serde(rename = "rootfs-image.update-module.single-file.mender_update_module")]
    RootfsImageUpdateModuleSingleFileMenderUpdateModule(Value<String>),
    #[serde(rename = "rootfs-image.update-module.single-file.version")]
    RootfsImageUpdateModuleSingleFileVersion(Value<String>),
    #[serde(rename = "rootfs-image.version")]
    RootfsImageVersion(Value<String>),
    #[serde(rename = "update_modules")]
    RootfsImageChecksumType(Value<Vec<String>>),
}
