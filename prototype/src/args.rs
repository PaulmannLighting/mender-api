use std::fs::read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::error;
use macaddr::MacAddr6;
use mender_api::Certificate;
use uuid::Uuid;

use crate::util::OrBail;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(help = "URL of the Mender server")]
    pub url: String,
    #[clap(long, short, help = "Username for Mender server login")]
    pub username: String,
    #[clap(long, short, help = "Password for Mender server login")]
    pub password: String,
    #[clap(long, short = 'k', help = "Accept invalid certificates (insecure)")]
    pub insecure: bool,
    #[clap(long, short, help = "Path to the certificate file (optional)")]
    pub certificate: Option<PathBuf>,
    #[clap(subcommand)]
    pub endpoint: Endpoint,
}

impl Args {
    /// Read the certificate file if it exists and parse it into a `Certificate`.
    pub fn certificate(&self) -> Result<Option<Certificate>, ExitCode> {
        self.certificate
            .as_ref()
            .and_then(|certificate| {
                read(certificate)
                    .inspect_err(|error| error!("Failed to read certificate file: {error}"))
                    .ok()
            })
            .map(|cert| Certificate::from_pem(&cert).or_bail())
            .transpose()
    }
}

#[derive(Debug, Subcommand)]
pub enum Endpoint {
    #[clap(name = "deployments")]
    Deployment {
        #[clap(subcommand)]
        action: Deployment,
    },
    #[clap(name = "devices")]
    Device {
        #[clap(subcommand)]
        action: Device,
    },
    #[clap(name = "groups")]
    Group {
        #[clap(subcommand)]
        action: Group,
    },
    #[clap(name = "releases")]
    Release {
        #[clap(subcommand)]
        action: Release,
    },
    #[clap(name = "device")]
    DeviceProxy {
        #[clap(help = "ID of the device to manage")]
        id: Uuid,
        #[clap(subcommand)]
        action: DeviceProxy,
    },
}

#[derive(Debug, Subcommand)]
pub enum Deployment {
    #[clap(name = "list")]
    List,
    #[clap(name = "device-of")]
    DevicesOf {
        #[clap(index = 1, help = "List device for a specific deployment")]
        id: Uuid,
    },
    #[clap(name = "add")]
    Add {
        #[clap(index = 1, help = "Name of the deployment")]
        name: String,
        #[clap(index = 2, help = "Artifact name for the deployment")]
        artifact_name: String,
        #[clap(long, short = 'D', help = "Devices to deploy")]
        devices: Vec<Uuid>,
        #[clap(long, short = 'R', help = "Number of retries for the deployment")]
        retries: usize,
    },
}

#[derive(Debug, Subcommand)]
pub enum Device {
    #[clap(name = "list")]
    List,
    Get {
        #[clap(index = 1, help = "ID of the device to retrieve")]
        id: Uuid,
    },
    AddToGroup {
        #[clap(index = 1, help = "ID of the device to add to a group")]
        id: Uuid,
        #[clap(index = 2, help = "Name of the group to add the device to")]
        group_name: String,
    },
    #[clap(name = "by-mac")]
    ByMac {
        #[clap(index = 1, help = "Find a device by its MAC address")]
        mac_address: MacAddr6,
    },
}

#[derive(Debug, Subcommand)]
pub enum Group {
    #[clap(name = "list")]
    List,
    #[clap(name = "device")]
    Devices {
        #[clap(index = 1, help = "List the device in a group")]
        name: String,
    },
    #[clap(name = "patch")]
    Patch {
        #[clap(help = "The name of the group to patch")]
        name: String,
        #[clap(help = "List of device IDs to add to the group")]
        devices: Vec<Uuid>,
    },
}

#[derive(Debug, Subcommand)]
pub enum Release {
    #[clap(name = "list")]
    List,
    #[clap(name = "by-name")]
    ByName {
        #[clap(index = 1, help = "Find a release by its name")]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceProxy {
    #[clap(name = "get")]
    Get,
    #[clap(name = "tag")]
    Tag {
        #[clap(subcommand)]
        action: TagAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum TagAction {
    #[clap(name = "get")]
    Add {
        #[clap(help = "Tag name")]
        name: String,
        #[clap(help = "Tag value")]
        value: String,
        #[clap(long, short = 'd', help = "Optional description for the tag")]
        description: Option<String>,
    },
}
