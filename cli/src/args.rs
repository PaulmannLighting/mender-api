use std::fs::read;
use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use log::error;
use mender_api::{Certificate, Devices, Session};
use uuid::Uuid;

use crate::args::deployments_action::DeploymentAction;
use crate::args::device_action::DeviceAction;
use crate::args::device_proxy_action::DeviceProxyAction;
use crate::args::group_action::GroupAction;
use crate::args::release_action::ReleaseAction;
use crate::util::OrBail;

mod deployments_action;
mod device_action;
mod device_proxy_action;
mod group_action;
mod release_action;

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

impl Endpoint {
    pub async fn run(self, session: &Session) -> Result<(), ExitCode> {
        match self {
            Self::Deployments { action } => action.run(session).await,
            Self::Devices { action } => action.run(session).await,
            Self::Groups { action } => action.run(session).await,
            Self::Releases { action } => action.run(session).await,
            Self::DeviceProxy { id, action } => action.run(session.proxy(id)).await,
        }
    }
}
