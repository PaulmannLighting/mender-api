//! Mender API data transfer objects (DTOs).

pub use self::artifact::{Artifact, Dependency, File, Info, TypeInfo, Update};
pub use self::attribute::{Attribute, KnownAttribute, UnknownAttribute};
pub use self::authentification_set::AuthentificationSet;
pub use self::bootloader_integration::BootloaderIntegration;
pub use self::country::Country;
pub use self::deployment::list::Deployment as ListDeployment;
pub use self::deployment::new::Deployment as NewDeployment;
pub use self::deployment::put::Deployment as PutDeployment;
pub use self::deployment::{Kind, Status as DeploymentStatus};
pub use self::device::{Device, Group as DeviceGroup};
pub use self::device_type::DeviceType;
pub use self::group::PatchGroupResponse;
pub use self::identity::Identity;
pub use self::release::Release;
pub use self::rootfs_type::RootfsType;
pub use self::scope::Scope;
pub use self::status::Status;
pub use self::tag::Tag;

mod artifact;
mod attribute;
mod authentification_set;
mod bootloader_integration;
mod country;
mod deployment;
mod device;
mod device_type;
mod group;
mod identity;
mod release;
mod rootfs_type;
mod scope;
mod status;
mod tag;
mod types;
