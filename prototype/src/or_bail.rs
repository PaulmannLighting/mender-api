use std::fmt::Display;
use std::process::ExitCode;

use log::error;

pub trait OrBail<T> {
    fn or_bail(self) -> Result<T, ExitCode>;
}

impl<T, E> OrBail<T> for Result<T, E>
where
    E: Display,
{
    fn or_bail(self) -> Result<T, ExitCode> {
        self.inspect_err(|error| error!("Error: {error}"))
            .map_err(|_| ExitCode::FAILURE)
    }
}
