//! Prototype for testing the API.

use std::process::ExitCode;

use args::Args;
use clap::Parser;
use mender_api::dto::NewDeployment;
use mender_api::{Client, Deployments, Devices, Groups, Login, Releases};

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
    let server = Client::new(args.url.parse().or_bail()?, cert, args.insecure).or_bail()?;
    let session = server.login(args.username, args.password).await.or_bail()?;

    match args.endpoint {
        Endpoint::Deployment { action } => match action {
            Deployment::List => {
                let mut deployments = Deployments::list(&session, None);

                while let Some(deployment) = deployments.next().await {
                    println!("{deployment:?}");
                }
            }
            Deployment::DevicesOf { id } => {
                for device_id in Deployments::devices_of(&session, id).await.or_bail()? {
                    println!("{device_id}");
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
                .or_bail()?;
            }
        },
        Endpoint::Device { action } => match action {
            Device::List => {
                let mut devices = Devices::list(&session, None);

                while let Some(device) = devices.next().await {
                    println!("{device:#}");
                }
            }
            Device::ByMac { mac_address } => {
                Devices::collect(&session, None)
                    .await
                    .or_bail()?
                    .into_iter()
                    .filter(|device| device.mac_address().is_some_and(|addr| addr == mac_address))
                    .for_each(|device| {
                        println!("{device}");
                    });
            }
        },
        Endpoint::Group { action } => match action {
            Group::List => {
                for group in Groups::list(&session).await.or_bail()? {
                    println!("{group}");
                }
            }
            Group::Devices { name } => {
                for device_id in Groups::devices_of(&session, &name, None).await.or_bail()? {
                    println!("{device_id}");
                }
            }
            Group::Patch { name, devices } => {
                let response = Groups::patch(&session, &name, &devices).await.or_bail()?;
                println!("{response:?}");
            }
        },
        Endpoint::Release { action } => match action {
            Release::List => {
                let mut releases = Releases::list(&session, None);

                while let Some(release) = releases.next().await {
                    println!("{release:?}");
                }
            }
            Release::ByName { name } => {
                Releases::collect(&session, None)
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
