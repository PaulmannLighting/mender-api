//! Prototype for testing the API.

use std::fs::read;

use args::Args;
use clap::Parser;
use log::error;
use mender_free_ext::api::dto::NewDeployment;
use mender_free_ext::{Api, Certificate, Deployments, Devices, Login, Releases};

use crate::args::{Deployment, Device, Endpoint, Release};

mod args;

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

    match args.endpoint {
        Endpoint::Deployment { action } => match action {
            Deployment::List => {
                for deployment in Deployments::list(&session)
                    .await
                    .expect("Failed to get deploxments.")
                {
                    println!("{deployment:?}");
                }
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
                for device in Devices::list(&session)
                    .await
                    .expect("Failed to get devices")
                {
                    println!("{device:?}");
                }
            }
            Device::ByMac { mac_address } => {
                if let Some(device) = Devices::list(&session)
                    .await
                    .expect("Failed to get devices")
                    .into_iter()
                    .find_map(|device| {
                        device.mac_address().and_then(|addr| {
                            if addr == mac_address {
                                Some(device)
                            } else {
                                None
                            }
                        })
                    })
                {
                    println!("Device: {device:?}");
                } else {
                    eprintln!("Error: Device not found");
                }
            }
        },
        Endpoint::Release { action } => match action {
            Release::List => {
                for release in Releases::list(&session)
                    .await
                    .expect("Failed to get releases.")
                {
                    println!("{release:?}");
                }
            }
            Release::ByName { name } => {
                if let Some(release) = Releases::list(&session)
                    .await
                    .expect("Failed to get releases.")
                    .into_iter()
                    .find(|release| release.name() == name)
                {
                    println!("Release: {release:?}");
                } else {
                    eprintln!("Error: Release not found");
                }
            }
        },
    }
}
