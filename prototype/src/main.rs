//! Prototype for testing the API.

use std::fs::read;
use std::path::PathBuf;

use clap::Parser;
use log::error;
use mender_free_ext::{Api, Certificate, Devices, Login, Releases};

#[derive(Debug, Parser)]
struct Args {
    #[clap(index = 1, help = "Username for Mender server login")]
    username: String,
    #[clap(index = 2, help = "Password for Mender server login")]
    password: String,
    #[clap(long, short, help = "Path to the certificate file (optional)")]
    certificate: Option<PathBuf>,
    #[clap(long, short = 'D', help = "List devices in the Mender server")]
    list_devices: bool,
    #[clap(long, short = 'R', help = "List releases in the Mender server")]
    list_releases: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let cert = args.certificate.and_then(|certificate| {
        read(certificate)
            .inspect_err(|error| error!("Failed to read certificate file: {error}"))
            .ok()
            .map(|cert| Certificate::from_pem(&cert).expect("Failed to parse certificate: {error}"))
    });

    let server = Api::new(
        "https://mender-acc.paulmann.com"
            .parse()
            .expect("Failed to parse base URL"),
        cert,
    )
    .expect("Failed to create MenderServer");

    let session = server
        .login(args.username, args.password)
        .await
        .expect("Failed to login MenderServer");

    if args.list_devices {
        for device in session
            .iter()
            .await
            .expect("Failed to get Mender deployments")
        {
            println!("{device:?}");
        }
    }

    if args.list_releases {
        for release in session.list().await.expect("Failed to get releases.") {
            println!("{release:?}");
        }
    }
}
