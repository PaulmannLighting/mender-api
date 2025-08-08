use std::fs::read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::error;
use macaddr::MacAddr6;
use mender_api::Certificate;
use mender_api::dto::Status;
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
    Deployment {
        #[clap(subcommand)]
        action: Deployment,
    },
    DevAuth {
        #[clap(subcommand)]
        action: DevAuth,
    },
    Device {
        #[clap(subcommand)]
        action: Device,
    },
    Group {
        #[clap(subcommand)]
        action: Group,
    },
    Release {
        #[clap(subcommand)]
        action: Release,
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
pub enum DevAuth {
    #[clap(name = "list")]
    List,
    #[clap(name = "set-status")]
    SetStatus {
        #[clap(help = "UUID of the device to set status for")]
        id: Uuid,
        #[clap(help = "Authentication UUID of the device")]
        auth_id: Uuid,
        #[clap(help = "Status to set for the device")]
        status: Status,
    },
}

#[derive(Debug, Subcommand)]
pub enum Device {
    #[clap(name = "list")]
    List,
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
