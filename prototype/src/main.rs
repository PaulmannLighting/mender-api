//! Prototype for testing the API.

use std::fs::read;

use args::Args;
use clap::Parser;
use log::error;
use mender_free_ext::api::dto::NewDeployment;
use mender_free_ext::{Api, Certificate, Deployments, Devices, Groups, Login, Releases};

use crate::args::{Deployment, Device, Endpoint, Group, Release};

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
                let mut deployments = Deployments::iter(&session);

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
                let mut devices = Devices::iter(&session);

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
        Endpoint::Group { action } => match action {
            Group::List => {
                for group in Groups::list(&session)
                    .await
                    .expect("Failed to list groups.")
                {
                    println!("{group}");
                }
            }
            Group::Devices { name } => {
                for device_id in session
                    .devices_of(&name)
                    .await
                    .expect("Failed to get devices.")
                {
                    println!("{device_id}");
                }
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
