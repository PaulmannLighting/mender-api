use serde::{Deserialize, Serialize};

/// Status of a device in the list.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Device is pending.
    Pending,
    /// Request has been accepted.
    Accepted,
    /// Missing authentication.
    NoAuth,
    /// Request was rejected.
    Rejected,
}
