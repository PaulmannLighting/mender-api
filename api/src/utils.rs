//! Common utility functions for formatting and displaying data.

use std::fmt::Display;

pub use self::pem_certificate::PemCertificate;
pub use self::response_ext::ResponseExt;

pub mod as_str;
mod pem_certificate;
mod response_ext;

/// Display a slice of items, formatting them as a comma-separated list enclosed in square brackets.
pub trait DisplaySlice {
    /// Display a slice of items, formatting them as a comma-separated list enclosed in square brackets.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T> DisplaySlice for [T]
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;

        let mut items = self.iter();

        if let Some(first) = items.next() {
            first.fmt(f)?;

            for item in items {
                write!(f, ", ")?;
                item.fmt(f)?;
            }
        }

        write!(f, "]")
    }
}
