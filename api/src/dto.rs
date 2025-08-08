//! Mender API data transfer objects (DTOs).

pub use artifact::{Artifact, Dependency, File, Info, TypeInfo, Update};
pub use attribute::Attribute;
pub use authentification_set::AuthentificationSet;
pub use bootloader_integration::BootloaderIntegration;
pub use country::Country;
pub use deployments::list::Deployment as ListDeployment;
pub use deployments::new::Deployment as NewDeployment;
pub use deployments::{Kind, Status as DeploymentStatus};
pub use dev_auth::Device as DevAuthDevice;
pub use device_type::DeviceType;
pub use devices::{Device, Group as DeviceGroup};
pub use groups::PatchGroupResponse;
pub use identity::Identity;
pub use releases::Release;
pub use rootfs_type::RootfsType;
pub use scope::Scope;
pub use status::Status;

mod artifact;
mod attribute;
mod authentification_set;
mod bootloader_integration;
mod country;
mod deployments;
mod dev_auth;
mod device_type;
mod devices;
mod groups;
mod identity;
mod releases;
mod rootfs_type;
mod scope;
mod status;
mod types;
