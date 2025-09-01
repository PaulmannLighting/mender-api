use std::num::NonZero;
use std::process::ExitCode;

use clap::Subcommand;
use log::error;
use mender_api::dto::DeploymentStatus;
use mender_api::{Deployments, Session};
use uuid::Uuid;

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum DeploymentAction {
    List {
        #[clap(long, short = 'p', help = "Page size for deployment listing")]
        page_size: Option<NonZero<usize>>,
        #[clap(long, short = 'v', help = "Verbose output")]
        verbose: bool,
        #[clap(long, short = 'u', help = "List only unfinished deployments")]
        unfinished: bool,
    },
    Show {
        #[clap(index = 1, help = "Show details for a specific deployment")]
        id: Uuid,
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
    AbortAllByDevice {
        #[clap(long, short = 'p', help = "Page size for deployment querying")]
        page_size: Option<NonZero<usize>>,
    },
}

impl DeploymentAction {
    pub async fn run(self, session: &Session) -> Result<(), ExitCode> {
        match self {
            Self::List {
                page_size,
                verbose,
                unfinished,
            } => {
                let mut deployments = Deployments::list(session, page_size);

                while let Some(result) = deployments.next().await {
                    match result {
                        Ok(deployment) => {
                            if !unfinished || deployment.status() != DeploymentStatus::Finished {
                                if verbose {
                                    println!("{deployment:?}");
                                } else {
                                    println!("{}", deployment.id());
                                }
                            }
                        }
                        Err(error) => {
                            error!("{error}");
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
            }
            Self::Show { id } => {
                let deployment = Deployments::show(session, id).await.or_bail()?;
                println!("{deployment:?}");
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
                Deployments::abort_all(session, page_size).await.or_bail()?;
            }
            Self::AbortAllByDevice { page_size } => {
                Deployments::abort_all_by_device(session, page_size)
                    .await
                    .or_bail()?;
            }
        }

        Ok(())
    }
}
