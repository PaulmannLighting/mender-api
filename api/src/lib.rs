//!  Implementation of the Mender server API.

pub use client::Client;
pub use deployments::Deployments;
pub use dev_auth::DevAuth;
pub use devices::Devices;
pub use groups::Groups;
pub use login::Login;
pub use releases::Releases;
pub use reqwest::Certificate;

mod client;
mod deployments;
mod dev_auth;
mod devices;
pub mod dto;
mod groups;
mod login;
mod pager;
mod releases;
mod session;
mod utils;
