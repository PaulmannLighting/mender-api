//! Data structures for listing devices.

mod as_str;
mod attribute;
mod mac_address;
mod one_or_many;
mod scope;
mod value;

use attribute::Attribute;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A list of devices in the Mender server.
pub type DeviceList = Vec<Device>;

/// A device in the Mender server.
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct Device {
    id: Uuid,
    attributes: Vec<Attribute>,
    updated_ts: DateTime<FixedOffset>,
}
