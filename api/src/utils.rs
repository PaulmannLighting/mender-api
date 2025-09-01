//! Common utility functions for formatting and displaying data.

use std::fmt::Display;

pub mod as_str;

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
