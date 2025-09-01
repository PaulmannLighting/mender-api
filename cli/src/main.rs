//! Prototype for testing the API.

use std::process::ExitCode;

use args::Args;
use clap::Parser;

use crate::util::IntoExitCode;

mod args;
mod util;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    Args::parse().run().await.into_exit_code()
}
