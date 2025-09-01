//! Paging of multi-page result listings.

use std::num::NonZero;

pub use paged_iterator::PagedIterator;
pub use pager::Pager;
pub use pages::Pages;

mod paged_iterator;
mod pager;
mod pages;

pub const DEFAULT_PAGE_SIZE: NonZero<usize> =
    NonZero::new(500).expect("Default page should be be non-zero.");
