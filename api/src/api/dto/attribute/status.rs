use std::fmt::Display;

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

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Accepted => write!(f, "accepted"),
            Self::NoAuth => write!(f, "noauth"),
            Self::Rejected => write!(f, "rejected"),
        }
    }
}
