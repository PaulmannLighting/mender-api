use std::num::NonZero;
use std::process::ExitCode;

use clap::Subcommand;
use log::{debug, error, info};
use mender_api::{Deployments, Devices, Session};
use tokio::time::Instant;
use uuid::Uuid;

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum DeploymentAction {
    List {
        #[clap(long, short = 'p', help = "Page size for deployment listing")]
        page_size: Option<NonZero<usize>>,
    },
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
    Abort {
        #[clap(index = 1, help = "ID of the deployment to abort")]
        id: Uuid,
    },
    AbortAll {
        #[clap(long, short = 'p', help = "Page size for deployment querying")]
        page_size: Option<NonZero<usize>>,
    },
}

impl DeploymentAction {
    pub async fn run(self, session: &Session) -> Result<(), ExitCode> {
        match self {
            Self::List { page_size } => {
                let mut deployments = Deployments::list(session, page_size);

                while let Some(result) = deployments.next().await {
                    match result {
                        Ok(deployment) => println!("{deployment:?}"),
                        Err(error) => {
                            error!("{error}");
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
            }
            Self::DevicesOf { id } => {
                for device_id in Deployments::devices_of(session, id).await.or_bail()? {
                    println!("{device_id}");
                }
            }
            Self::Add {
                name,
                artifact_name,
                devices,
                retries,
            } => {
                Deployments::create(session, name, artifact_name, &devices, retries)
                    .await
                    .or_bail()?;
            }
            Self::Abort { id } => {
                Deployments::abort(session, id).await.or_bail()?;
            }
            Self::AbortAll { page_size } => {
                let start = Instant::now();
                let devices = Devices::collect(session, page_size).await.or_bail()?;
                debug!(
                    "Collected {} devices in {:?}",
                    devices.len(),
                    start.elapsed()
                );
                let mut handles = Vec::with_capacity(devices.len());

                for device in devices {
                    let my_session = session.clone();
                    handles.push(tokio::spawn(async move {
                        if let Err(error) =
                            Deployments::abort_device(&my_session, device.id()).await
                        {
                            error!("Failed to abort deployment for device {device}: {error}");
                        } else {
                            info!("Aborted deployment for device {device}");
                        }
                    }));
                }

                for handle in handles {
                    let _ = handle.await;
                }
            }
        }

        Ok(())
    }
}
