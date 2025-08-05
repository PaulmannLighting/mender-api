//! Prototype for testing the API.

use std::process::ExitCode;

use args::Args;
use clap::Parser;
use mender_free_ext::api::dto::NewDeployment;
use mender_free_ext::{Api, Deployments, Devices, Groups, Login, Releases};

use crate::args::{Deployment, Device, Endpoint, Group, Release};
use crate::or_bail::OrBail;

mod args;
mod or_bail;

#[allow(clippy::too_many_lines)]
#[tokio::main]
async fn main() -> Result<(), ExitCode> {
    env_logger::init();

    let args = Args::parse();
    let cert = args.certificate()?;

    let server = Api::new("https://mender-acc.paulmann.com".parse().or_bail()?, cert).or_bail()?;

    let session = server.login(args.username, args.password).await.or_bail()?;

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
                .or_bail()?;
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
                for group in Groups::list(&session).await.or_bail()? {
                    println!("{group}");
                }
            }
            Group::Devices { name } => {
                for device_id in session.devices_of(&name).await.or_bail()? {
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
