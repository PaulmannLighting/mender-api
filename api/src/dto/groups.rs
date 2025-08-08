//! DTOs for the groups API.

use serde::{Deserialize, Serialize};

/// Response from a patch operation on a group.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PatchGroupResponse {
    #[serde(rename = "matched_count")]
    matched: usize,
    #[serde(rename = "updated_count")]
    updated: usize,
}

impl PatchGroupResponse {
    /// Returns the number of groups matched by the patch operation.
    #[must_use]
    pub const fn matched(&self) -> usize {
        self.matched
    }

    /// Returns the number of groups updated by the patch operation.
    #[must_use]
    pub const fn updated(&self) -> usize {
        self.updated
    }
}
