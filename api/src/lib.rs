//! API to the mender server extensions.

pub use api::{ApiVersion, Deployments, Devauth, Inventory, Management, MenderServer, UserAdm};

mod api;
mod util;
