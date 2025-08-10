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
    Deployments {
        #[clap(subcommand)]
        action: DeploymentAction,
    },
    Devices {
        #[clap(subcommand)]
        action: DeviceAction,
    },
    Groups {
        #[clap(subcommand)]
        action: GroupAction,
    },
    Releases {
        #[clap(subcommand)]
        action: ReleaseAction,
    },
    #[clap(name = "device")]
    DeviceProxy {
        #[clap(help = "ID of the device to manage")]
        id: Uuid,
        #[clap(subcommand)]
        action: DeviceProxyAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeploymentAction {
    List,
    DevicesOf {
        #[clap(index = 1, help = "List device for a specific deployment")]
        id: Uuid,
    },
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
pub enum DeviceAction {
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
    ByMac {
        #[clap(index = 1, help = "Find a device by its MAC address")]
        mac_address: MacAddr6,
    },
}

#[derive(Debug, Subcommand)]
pub enum GroupAction {
    List,
    Devices {
        #[clap(index = 1, help = "List the device in a group")]
        name: String,
    },
    Patch {
        #[clap(help = "The name of the group to patch")]
        name: String,
        #[clap(help = "List of device IDs to add to the group")]
        devices: Vec<Uuid>,
    },
}

#[derive(Debug, Subcommand)]
pub enum ReleaseAction {
    List,
    ByName {
        #[clap(index = 1, help = "Find a release by its name")]
        name: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum DeviceProxyAction {
    Get,
    Tag {
        #[clap(subcommand)]
        action: TagAction,
    },
}

#[derive(Debug, Subcommand)]
pub enum TagAction {
    Assign {
        #[clap(help = "Tag name")]
        name: String,
        #[clap(help = "Tag value")]
        value: String,
        #[clap(long, short = 'd', help = "Optional description for the tag")]
        description: Option<String>,
    },
}
