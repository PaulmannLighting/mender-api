//! API to the mender server extensions.

pub use api::{ApiVersion, Deployments, Devauth, Inventory, Management, MenderServer, UserAdm};
pub use reqwest::Certificate;

mod api;
mod util;
