//! Common utility functions for formatting and displaying data.

use std::fmt::Display;

pub mod as_str;

/// Display a slice of items, formatting them as a comma-separated list enclosed in square brackets.
pub fn display_slice<T>(items: &[T], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
where
    T: Display,
{
    write!(f, "[")?;

    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            write!(f, ", ")?;
        }

        item.fmt(f)?;
    }

    write!(f, "]")
}
