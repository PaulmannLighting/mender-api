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
                for deployment in Deployments::collect(&session)
                    .await
                    .expect("Failed to collect deployments")
                {
                    println!("{deployment:?}");
                }
                let mut deployments = Deployments::list(&session);

                while let Some(deployment) = deployments.next().await {
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
                let mut devices = Devices::list(&session);

                while let Some(device) = devices.next().await {
                    println!("{device:?}");
                }
            }
            Device::ByMac { mac_address } => {
                Devices::collect(&session)
                    .await
                    .expect("Failed to collect devices.")
                    .into_iter()
                    .filter(|device| device.mac_address().is_some_and(|addr| addr == mac_address))
                    .for_each(|device| {
                        println!("Device: {device:?}");
                    });
            }
        },
        Endpoint::Release { action } => match action {
            Release::List => {
                let mut releases = Releases::list(&session);

                while let Some(release) = releases.next().await {
                    println!("{release:?}");
                }
            }
            Release::ByName { name } => {
                Releases::collect(&session)
                    .await
                    .expect("Failed to collect releases")
                    .into_iter()
                    .filter(|release| release.name() == name)
                    .for_each(|release| {
                        println!("Release: {release:?}");
                    });
            }
        },
    }
}
