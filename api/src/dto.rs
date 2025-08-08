//! Mender API data transfer objects (DTOs).

pub use artifact::{Artifact, Dependency, File, Info, TypeInfo, Update};
pub use attribute::{Attribute, KnownAttribute};
pub use authentification_set::AuthentificationSet;
pub use bootloader_integration::BootloaderIntegration;
pub use country::Country;
pub use deployment::list::Deployment as ListDeployment;
pub use deployment::new::Deployment as NewDeployment;
pub use deployment::{Kind, Status as DeploymentStatus};
pub use dev_auth::Device as DevAuthDevice;
pub use device::{Device, Group as DeviceGroup};
pub use device_type::DeviceType;
pub use group::PatchGroupResponse;
pub use identity::Identity;
pub use release::Release;
pub use rootfs_type::RootfsType;
pub use scope::Scope;
pub use status::Status;

mod artifact;
mod attribute;
mod authentification_set;
mod bootloader_integration;
mod country;
mod deployment;
mod dev_auth;
mod device;
mod device_type;
mod group;
mod identity;
mod release;
mod rootfs_type;
mod scope;
mod status;
mod types;
