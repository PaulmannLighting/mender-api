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
        self.map_err(|error| {
            error!("{error}");
            ExitCode::FAILURE
        })
    }
}

pub trait IntoExitCode {
    fn into_exit_code(self) -> ExitCode;
}

impl IntoExitCode for Result<(), ExitCode> {
    fn into_exit_code(self) -> ExitCode {
        match self {
            Ok(()) => ExitCode::SUCCESS,
            Err(exit_code) => exit_code,
        }
    }
}
