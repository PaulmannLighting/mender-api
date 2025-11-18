//! Paging of multi-page result listings.

use std::num::NonZero;

pub use self::pager::Pager;
pub use self::pages::Pages;
pub use self::paginated_iterator::PaginatedIterator;

mod pager;
mod pages;
mod paginated_iterator;

pub const DEFAULT_PAGE_SIZE: NonZero<usize> =
    NonZero::new(500).expect("Default page should be be non-zero.");
