//! API to the mender server extensions.

pub use api::{DeviceList, Devices, Login, MenderServer};
pub use reqwest::Certificate;

pub mod api;
