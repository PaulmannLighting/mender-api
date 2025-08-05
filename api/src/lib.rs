//! API to the mender server extensions.

pub use api::{Api, Deployments, Devices, Login, Releases};
pub use reqwest::Certificate;

pub mod api;
