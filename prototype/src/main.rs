//! Prototype for testing the API.

use std::process::ExitCode;

use args::Args;
use clap::Parser;
use mender_free_ext::api::dto::NewDeployment;
use mender_free_ext::{Api, Deployments, Devices, Groups, Login, Releases};

use crate::args::{Deployment, Device, Endpoint, Group, Release};
use crate::util::{IntoExitCode, OrBail};

mod args;
mod util;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    run(Args::parse()).await.into_exit_code()
}

async fn run(args: Args) -> Result<(), ExitCode> {
    let cert = args.certificate()?;

    let server = Api::new("https://mender-acc.paulmann.com".parse().or_bail()?, cert).or_bail()?;

    let session = server.login(args.username, args.password).await.or_bail()?;

    match args.endpoint {
        Endpoint::Deployment { action } => match action {
            Deployment::List => {
                let mut deployments = session.deployments().iter();

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
                session
                    .deployments()
                    .create(&NewDeployment::new(name, artifact_name, devices, retries))
                    .await
                    .or_bail()?;
            }
        },
        Endpoint::Device { action } => match action {
            Device::List => {
                let mut devices = session.devices().iter();

                while let Some(device) = devices.next().await {
                    println!("{device:?}");
                }
            }
            Device::ByMac { mac_address } => {
                session
                    .devices()
                    .collect()
                    .await
                    .or_bail()?
                    .into_iter()
                    .filter(|device| device.mac_address().is_some_and(|addr| addr == mac_address))
                    .for_each(|device| {
                        println!("Device: {device:?}");
                    });
            }
        },
        Endpoint::Group { action } => match action {
            Group::List => {
                for group in session.groups().list().await.or_bail()? {
                    println!("{group}");
                }
            }
            Group::Devices { name } => {
                for device_id in session.groups().devices_of(&name).await.or_bail()? {
                    println!("{device_id}");
                }
            }
        },
        Endpoint::Release { action } => match action {
            Release::List => {
                let mut releases = session.releases().list();

                while let Some(release) = releases.next().await {
                    println!("{release:?}");
                }
            }
            Release::ByName { name } => {
                session
                    .releases()
                    .collect()
                    .await
                    .or_bail()?
                    .into_iter()
                    .filter(|release| release.name() == name)
                    .for_each(|release| {
                        println!("Release: {release:?}");
                    });
            }
        },
    }

    Ok(())
}
