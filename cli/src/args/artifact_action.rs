use std::num::NonZero;
use std::process::ExitCode;

use clap::Subcommand;
use log::error;
use mender_api::{Artifacts, Session};
use uuid::Uuid;

#[derive(Debug, Subcommand)]
pub enum ArtifactAction {
    List {
        #[clap(long, short = 'p', help = "Page size for device listing")]
        page_size: Option<NonZero<usize>>,
        #[clap(long, short = 'v', help = "List detailed device information")]
        verbose: bool,
    },
    Delete {
        #[clap(index = 1, help = "Delete an artifact by its ID")]
        id: Uuid,
    },
}

impl ArtifactAction {
    pub async fn run(self, session: &Session) -> Result<(), ExitCode> {
        match self {
            Self::List { page_size, verbose } => {
                let mut devices = Artifacts::list(session, page_size);

                while let Some(result) = devices.next().await {
                    match result {
                        Ok(artifact) => {
                            if verbose {
                                println!("{artifact:#}");
                            } else {
                                println!("{artifact}");
                            }
                        }
                        Err(error) => {
                            error!("{error}");
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
            }
            Self::Delete { id } => {
                Artifacts::delete(session, id).await.map_err(|error| {
                    error!("{error}");
                    ExitCode::FAILURE
                })?;
            }
        }

        Ok(())
    }
}
