use serde::{Deserialize, Serialize};

/// Possible deployment status.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// Deployment is pending.
    Pending,
    /// Deployment is in progress.
    InProgress,
    /// Deployment has finished.
    Finished,
}
