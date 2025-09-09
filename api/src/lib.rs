//!  Implementation of the Mender server API.

pub use client::Client;
pub use deployments::Deployments;
pub use device_proxy::DeviceProxy;
pub use devices::Devices;
pub use groups::Groups;
pub use login::Login;
pub use paging::{PagedIterator, Pager, Pages};
pub use releases::Releases;
pub use reqwest::{Certificate, Error, Result};
pub use session::Session;
pub use tags::Tags;
pub use utils::PemCertificate;

mod client;
mod deployments;
mod device_proxy;
mod devices;
pub mod dto;
mod groups;
mod login;
mod paging;
mod releases;
mod session;
mod tags;
mod utils;
