use std::num::NonZero;
use std::process::ExitCode;

use clap::Subcommand;
use log::error;
use macaddr::MacAddr6;
use mender_api::{Artifacts, Session};
use uuid::Uuid;

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum ArtifactAction {
    List {
        #[clap(long, short = 'p', help = "Page size for device listing")]
        page_size: Option<NonZero<usize>>,
        #[clap(long, short = 'p', help = "List detailed device information")]
        verbose: bool,
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
        }

        Ok(())
    }
}
