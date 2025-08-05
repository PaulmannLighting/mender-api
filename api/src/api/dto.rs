//! Mender API data transfer objects (DTOs).

pub use artifact::{Artifact, Dependency, File, Info, TypeInfo, Update};
pub use attribute::{Attribute, Scope, Status as AttributeStatus, Value};
pub use bootloader_integration::BootloaderIntegration;
pub use country::Country;
pub use deployments::list::{Deployment as ListDeployment, DeploymentList};
pub use deployments::new::Deployment as NewDeployment;
pub use deployments::{Kind, Status as DeploymentStatus};
pub use device_type::DeviceType;
pub use devices::{Device, DeviceList};
pub use releases::{Release, ReleaseList};
pub use rootfs_type::RootfsType;

mod artifact;
mod attribute;
mod bootloader_integration;
mod country;
mod deployments;
mod device_type;
mod devices;
mod releases;
mod rootfs_type;
mod types;
