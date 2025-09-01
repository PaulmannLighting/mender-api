use std::num::NonZero;
use std::process::ExitCode;

use clap::Subcommand;
use mender_api::{Releases, Session};

use crate::util::OrBail;

#[derive(Debug, Subcommand)]
pub enum ReleaseAction {
    List {
        #[clap(long, short = 'p', help = "Page size for releases listing")]
        page_size: Option<NonZero<usize>>,
    },
    ByName {
        #[clap(index = 1, help = "Find a release by its name")]
        name: String,
        #[clap(long, short = 'p', help = "Page size for releases listing")]
        page_size: Option<NonZero<usize>>,
    },
}

impl ReleaseAction {
    pub async fn run(self, session: &Session) -> Result<(), ExitCode> {
        match self {
            Self::List { page_size } => {
                let mut releases = Releases::list(session, page_size);

                while let Some(release) = releases.next().await {
                    println!("{release:?}");
                }
            }
            Self::ByName { name, page_size } => {
                Releases::collect(session, page_size)
                    .await
                    .or_bail()?
                    .into_iter()
                    .filter(|release| release.name() == name)
                    .for_each(|release| {
                        println!("Release: {release:?}");
                    });
            }
        }

        Ok(())
    }
}
