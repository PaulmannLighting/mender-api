use std::path::Path;

/// Trait for accessing configuration arguments.
pub trait ConfigArgs {
    /// Mender server URL.
    #[must_use]
    fn url(&self) -> Option<&str>;

    /// Path to the client certificate.
    #[must_use]
    fn certificate(&self) -> Option<&Path>;

    /// Username for authentication.
    #[must_use]
    fn username(&self) -> Option<&str>;

    /// Password for authentication.
    #[must_use]
    fn password(&self) -> Option<&str>;

    /// Whether to skip TLS verification.
    #[must_use]
    fn insecure(&self) -> bool;
}
