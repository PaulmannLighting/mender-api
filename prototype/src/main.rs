//! Prototype for testing the API.

use std::process::ExitCode;

use args::Args;
use clap::Parser;
use log::error;
use mender_api::dto::{NewDeployment, Tag};
use mender_api::{Client, Deployments, Devices, Groups, Login, Releases};

use crate::args::{
    DeploymentAction, DeviceAction, DeviceProxyAction, Endpoint, GroupAction, ReleaseAction,
    TagsAction,
};
use crate::util::{IntoExitCode, OrBail};

mod args;
mod util;

#[tokio::main]
async fn main() -> ExitCode {
    env_logger::init();
    run(Args::parse()).await.into_exit_code()
}

#[allow(clippy::cognitive_complexity)]
#[allow(clippy::too_many_lines)]
async fn run(args: Args) -> Result<(), ExitCode> {
    let cert = args.certificate()?;
    let server = Client::new(args.url.parse().or_bail()?, cert, args.insecure).or_bail()?;
    let session = server.login(args.username, args.password).await.or_bail()?;

    match args.endpoint {
        Endpoint::Deployments { action } => match action {
            DeploymentAction::List => {
                let mut deployments = Deployments::list(&session, None);

                while let Some(result) = deployments.next().await {
                    match result {
                        Ok(deployment) => println!("{deployment:?}"),
                        Err(error) => {
                            error!("{error}");
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
            }
            DeploymentAction::DevicesOf { id } => {
                for device_id in Deployments::devices_of(&session, id).await.or_bail()? {
                    println!("{device_id}");
                }
            }
            DeploymentAction::Add {
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
            DeploymentAction::Abort { id } => {
                Deployments::abort(&session, id).await.or_bail()?;
            }
            DeploymentAction::AbortAll => {
                Deployments::abort_all(&session).await.or_bail()?;
            }
        },
        Endpoint::Devices { action } => match action {
            DeviceAction::List => {
                let mut devices = Devices::list(&session, None);

                while let Some(result) = devices.next().await {
                    match result {
                        Ok(device) => println!("{device:#}"),
                        Err(error) => {
                            error!("{error}");
                            return Err(ExitCode::FAILURE);
                        }
                    }
                }
            }
            DeviceAction::Get { id } => {
                let device = Devices::get(&session, id).await.or_bail()?;
                println!("{device:#}");
            }
            DeviceAction::AddToGroup { id, group_name } => {
                Devices::add_to_group(&session, id, group_name)
                    .await
                    .or_bail()?;
            }
            DeviceAction::ByMac { mac_address } => {
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
        Endpoint::Groups { action } => match action {
            GroupAction::List => {
                for group in Groups::list(&session).await.or_bail()? {
                    println!("{group}");
                }
            }
            GroupAction::Devices { name } => {
                for device_id in Groups::devices_of(&session, &name, None).await.or_bail()? {
                    println!("{device_id}");
                }
            }
            GroupAction::Patch { name, devices } => {
                let response = Groups::patch(&session, &name, &devices).await.or_bail()?;
                println!("{response:?}");
            }
        },
        Endpoint::Releases { action } => match action {
            ReleaseAction::List => {
                let mut releases = Releases::list(&session, None);

                while let Some(release) = releases.next().await {
                    println!("{release:?}");
                }
            }
            ReleaseAction::ByName { name } => {
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
        Endpoint::DeviceProxy { id, action } => {
            let device = session.proxy(id);

            match action {
                DeviceProxyAction::Get => {
                    println!("{}", device.get().await.or_bail()?);
                }
                DeviceProxyAction::Tags { action } => match action {
                    TagsAction::Add {
                        name,
                        value,
                        description,
                    } => {
                        device
                            .add_tags(&[Tag::new(name, value, description)])
                            .await
                            .or_bail()?;
                    }
                    TagsAction::Assign {
                        name,
                        value,
                        description,
                    } => {
                        device
                            .assign_tags(&[Tag::new(name, value, description)])
                            .await
                            .or_bail()?;
                    }
                    TagsAction::Clear => {
                        device.clear_tags().await.or_bail()?;
                    }
                },
            }
        }
    }

    Ok(())
}
