//! Prototype for testing the API.

use std::fs::read;

use args::Args;
use clap::Parser;
use log::error;
use mender_free_ext::api::dto::NewDeployment;
use mender_free_ext::{Api, Certificate, Deployments, Devices, Login, Releases, async_iter};

use crate::args::{Deployment, Device, Endpoint, Release};

mod args;

#[tokio::main]
async fn main() {
    env_logger::init();

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

    match args.endpoint {
        Endpoint::Deployment { action } => match action {
            Deployment::List => {
                async_iter!(deployment in Deployments::list(&session) => {
                    println!("{deployment:?}");
                });
            }
            Deployment::Add {
                name,
                artifact_name,
                devices,
                retries,
            } => {
                Deployments::create(
                    &session,
                    &NewDeployment::new(name, artifact_name, devices, retries),
                )
                .await
                .expect("Failed to create deployment");
            }
        },
        Endpoint::Device { action } => match action {
            Device::List => {
                async_iter!( device in Devices::list(&session) => {
                    println!("{device:?}");
                });
            }
            Device::ByMac { mac_address } => {
                async_iter!(device in Devices::list(&session) => {
                    if device.mac_address().is_some_and(|addr| addr == mac_address) {
                        println!("Device: {device:?}");
                    } else {
                        eprintln!("Error: Device not found");
                    }
                });
            }
        },
        Endpoint::Release { action } => match action {
            Release::List => {
                async_iter!(release in  Releases::list(&session) => {
                    println!("{release:?}");
                });
            }
            Release::ByName { name } => {
                async_iter!(release in Releases::list(&session) => {
                    if release.name() == name {
                        println!("Release: {release:?}");
                    } else {
                        eprintln!("Error: Release not found");
                    }
                });
            }
        },
    }
}
