use std::num::NonZero;

use clap::Subcommand;
use mender_api::Groups;
use uuid::Uuid;

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum GroupAction {
    List,
    Devices {
        #[clap(index = 1, help = "List the device in a group")]
        name: String,
        #[clap(long, short = 'p', help = "Page size for group listing")]
        page_size: Option<NonZero<usize>>,
    },
    Patch {
        #[clap(help = "The name of the group to patch")]
        name: String,
        #[clap(help = "List of device IDs to add to the group")]
        devices: Vec<Uuid>,
    },
}

impl GroupAction {
    pub async fn run(self, session: &mender_api::Session) -> Result<(), std::process::ExitCode> {
        match self {
            Self::List => {
                for group in Groups::list(session).await.or_bail()? {
                    println!("{group}");
                }
            }
            Self::Devices { name, page_size } => {
                for device_id in Groups::devices_of(session, &name, page_size)
                    .await
                    .or_bail()?
                {
                    println!("{device_id}");
                }
            }
            Self::Patch { name, devices } => {
                let response = Groups::patch(session, &name, &devices).await.or_bail()?;
                println!("{response:?}");
            }
        }

        Ok(())
    }
}
