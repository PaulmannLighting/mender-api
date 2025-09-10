//! Common utility functions for formatting and displaying data.

use std::fmt::Display;

use clap::builder::TypedValueParser;
pub use pem_certificate::PemCertificate;
pub use response_ext::ResponseExt;

pub mod as_str;
mod pem_certificate;
mod response_ext;

/// Display a slice of items, formatting them as a comma-separated list enclosed in square brackets.
pub fn display_slice<T>(slice: &[T], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    T: Display,
{
    write!(f, "[")?;

    let mut items = slice.iter();

    if let Some(first) = items.next() {
        first.fmt(f)?;

        for item in items {
            write!(f, ", ")?;
            item.fmt(f)?;
        }
    }

    write!(f, "]")
}
