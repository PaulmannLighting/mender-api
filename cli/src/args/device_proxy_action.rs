use std::process::ExitCode;

use clap::Subcommand;
use mender_api::DeviceProxy;
use tags_action::TagsAction;

use crate::util::OrBail;

mod tags_action;

#[derive(Debug, Subcommand)]
pub enum DeviceProxyAction {
    Get,
    Tags {
        #[clap(subcommand)]
        action: TagsAction,
    },
}

impl DeviceProxyAction {
    pub async fn run(self, device: DeviceProxy<'_>) -> Result<(), ExitCode> {
        match self {
            Self::Get => {
                println!("{}", device.get().await.or_bail()?);
                Ok(())
            }
            Self::Tags { action } => action.run(device).await,
        }
    }
}
