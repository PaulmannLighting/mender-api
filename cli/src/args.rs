use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand};
use mender_api::{Devices, Session};
use mender_api_cfg::ConfigArgs;
use uuid::Uuid;

use self::artifact_action::ArtifactAction;
use crate::args::deployments_action::DeploymentAction;
use crate::args::device_action::DeviceAction;
use crate::args::device_proxy_action::DeviceProxyAction;
use crate::args::group_action::GroupAction;
use crate::args::release_action::ReleaseAction;

mod artifact_action;
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
    pub async fn run(self) -> Result<(), ExitCode> {
        let session = self.login().await?;
        self.endpoint.run(&session).await
    }
}

impl ConfigArgs for Args {
    fn url(&self) -> Option<&str> {
        Some(&self.url)
    }

    fn certificate(&self) -> Option<&Path> {
        self.certificate.as_deref()
    }

    fn username(&self) -> Option<&str> {
        Some(&self.username)
    }

    fn password(&self) -> Option<&str> {
        Some(&self.password)
    }

    fn insecure(&self) -> bool {
        self.insecure
    }
}

#[derive(Debug, Subcommand)]
pub enum Endpoint {
    Artifacts {
        #[clap(subcommand)]
        action: ArtifactAction,
    },
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
            Self::Artifacts { action } => action.run(session).await,
            Self::Deployments { action } => action.run(session).await,
            Self::Devices { action } => action.run(session).await,
            Self::Groups { action } => action.run(session).await,
            Self::Releases { action } => action.run(session).await,
            Self::DeviceProxy { id, action } => action.run(session.proxy(id)).await,
        }
    }
}
