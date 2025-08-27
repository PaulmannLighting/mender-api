//! Payload for deployment PUT requests.

use serde::Serialize;

use crate::dto::Status;

/// A new deployment put request.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Deployment {
    status: Status,
}

impl Deployment {
    /// Creates a new `Deployment` instance.
    #[must_use]
    pub const fn new(status: Status) -> Self {
        Self { status }
    }
}
