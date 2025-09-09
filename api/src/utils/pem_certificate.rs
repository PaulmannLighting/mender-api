use std::fs::read;
use std::path::Path;
use std::process::ExitCode;

use log::error;
use reqwest::Certificate;

/// Trait to load a TLS certificate from an optional file path.
pub trait PemCertificate {
    /// Load the certificate if the path is provided.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(Certificate))` if the certificate was successfully loaded and parsed.
    /// - `Ok(None)` if no path was provided.
    /// - `Err(ExitCode)` if there was an error reading or parsing the certificate.
    ///
    /// # Errors
    ///
    /// Returns `ExitCode::FAILURE` if reading the file or parsing the certificate fails.
    fn load(&self) -> Result<Option<Certificate>, ExitCode>;
}

impl<T> PemCertificate for Option<T>
where
    T: AsRef<Path>,
{
    fn load(&self) -> Result<Option<Certificate>, ExitCode> {
        self.as_ref()
            .and_then(|certificate| {
                read(certificate)
                    .inspect_err(|error| error!("Failed to read certificate file: {error}"))
                    .ok()
            })
            .map(|cert| {
                Certificate::from_pem(&cert)
                    .inspect_err(|error| error!("Failed to parse certificate: {error}"))
                    .map_err(|_| ExitCode::FAILURE)
            })
            .transpose()
    }
}
