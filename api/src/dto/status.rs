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

impl Status {
    /// Return a static string representation of the status.
    #[must_use]
    pub const fn to_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Accepted => "accepted",
            Self::NoAuth => "noauth",
            Self::Rejected => "rejected",
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(feature = "clap")]
impl clap::ValueEnum for Status {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Pending, Self::Accepted, Self::NoAuth, Self::Rejected]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(clap::builder::PossibleValue::new(self.to_str()))
    }
}
