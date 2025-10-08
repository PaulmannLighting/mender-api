//! Paging of multi-page result listings.

use std::num::NonZero;

pub use pager::Pager;
pub use pages::Pages;
pub use paginated_iterator::PaginatedIterator;

mod pager;
mod pages;
mod paginated_iterator;

pub const DEFAULT_PAGE_SIZE: NonZero<usize> =
    NonZero::new(500).expect("Default page should be be non-zero.");
