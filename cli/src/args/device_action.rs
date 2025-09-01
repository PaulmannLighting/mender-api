use std::num::NonZero;
use std::process::ExitCode;

use clap::Subcommand;
use log::error;
use macaddr::MacAddr6;
use mender_api::{Devices, Session};
use uuid::Uuid;

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum DeviceAction {
    List {
        #[clap(long, short = 'p', help = "Page size for device listing")]
        page_size: Option<NonZero<usize>>,
        #[clap(long, short = 'p', help = "List detailed device information")]
        verbose: bool,
    },
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
        #[clap(long, short = 'p', help = "Page size for device listing")]
        page_size: Option<NonZero<usize>>,
    },
}

impl DeviceAction {
    pub async fn run(self, session: &Session) -> Result<(), ExitCode> {
        match self {
            Self::List { page_size, verbose } => {
                let mut devices = Devices::list(session, page_size);

                while let Some(result) = devices.next().await {
                    match result {
                        Ok(device) => {
                            if verbose {
                                println!("{device:#}");
                            } else {
                                println!("{device}");
                            }
                        }
                        Err(error) => {
                            error!("{error}");
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
            }
            Self::Get { id } => {
                let device = Devices::get(session, id).await.or_bail()?;
                println!("{device:#}");
            }
            Self::AddToGroup { id, group_name } => {
                Devices::set_group(session, id, group_name)
                    .await
                    .or_bail()?;
            }
            Self::ByMac {
                mac_address,
                page_size,
            } => {
                Devices::collect(session, page_size)
                    .await
                    .or_bail()?
                    .into_iter()
                    .filter(|device| device.mac_address().is_some_and(|addr| addr == mac_address))
                    .for_each(|device| {
                        println!("{device}");
                    });
            }
        }

        Ok(())
    }
}
