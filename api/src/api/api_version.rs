use std::fmt::Display;

/// Represents the API version of the Mender server.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ApiVersion {
    V1,
    V2,
}

impl ApiVersion {
    /// Returns the API version as a string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            ApiVersion::V1 => "v1",
            ApiVersion::V2 => "v2",
        }
    }
}

impl Display for ApiVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
