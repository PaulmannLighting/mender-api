use std::process::ExitCode;

use clap::Subcommand;
use mender_api::DeviceProxy;
use mender_api::dto::Tag;

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum TagsAction {
    Add {
        #[clap(help = "Tag name")]
        name: String,
        #[clap(help = "Tag value")]
        value: String,
        #[clap(long, short = 'd', help = "Optional description for the tag")]
        description: Option<String>,
    },
    Assign {
        #[clap(help = "Tag name")]
        name: String,
        #[clap(help = "Tag value")]
        value: String,
        #[clap(long, short = 'd', help = "Optional description for the tag")]
        description: Option<String>,
    },
    Clear,
}

impl TagsAction {
    pub async fn run(self, device: DeviceProxy<'_>) -> Result<(), ExitCode> {
        match self {
            Self::Add {
                name,
                value,
                description,
            } => {
                device
                    .add_tags(&[Tag::new(name, value, description)])
                    .await
                    .or_bail()?;
            }
            Self::Assign {
                name,
                value,
                description,
            } => {
                device
                    .assign_tags(&[Tag::new(name, value, description)])
                    .await
                    .or_bail()?;
            }
            Self::Clear => {
                device.clear_tags().await.or_bail()?;
            }
        }

        Ok(())
    }
}
