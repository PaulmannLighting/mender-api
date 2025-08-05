use std::path::PathBuf;

use clap::{Parser, Subcommand};
use macaddr::MacAddr6;
use uuid::Uuid;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(index = 1, help = "Username for Mender server login")]
    pub username: String,
    #[clap(index = 2, help = "Password for Mender server login")]
    pub password: String,
    #[clap(long, short, help = "Path to the certificate file (optional)")]
    pub certificate: Option<PathBuf>,
    #[clap(subcommand)]
    pub endpoint: Endpoint,
}

#[derive(Debug, Subcommand)]
pub enum Endpoint {
    #[clap(name = "deployment")]
    Deployment {
        #[clap(subcommand)]
        action: Deployment,
    },
    Device {
        #[clap(subcommand)]
        action: Device,
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
    #[clap(name = "by-mac")]
    ByMac {
        #[clap(index = 1, help = "Find a device by its MAC address")]
        mac_address: MacAddr6,
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
