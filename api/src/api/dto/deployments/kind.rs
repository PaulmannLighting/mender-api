use serde::{Deserialize, Serialize};

/// Possible kinds of deployment.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    /// A software deployment.
    Software,
}
