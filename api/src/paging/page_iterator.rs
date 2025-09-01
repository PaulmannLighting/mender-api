use std::num::NonZero;
use std::vec::IntoIter;

use log::error;
use serde::Deserialize;

use crate::Pager;

/// Iterator for paginated results.
#[derive(Debug, Clone)]
pub struct PageIterator<'session, 'path, T> {
    pager: Pager<'session, 'path>,
    page_no: NonZero<usize>,
    current_page: Option<IntoIter<T>>,
}

impl<'session, 'path, T> PageIterator<'session, 'path, T> {
    /// Create a new page iterator with the given page size.
    #[must_use]
    pub(crate) const fn new(pager: Pager<'session, 'path>) -> Self {
        Self {
            pager,
            page_no: NonZero::new(1).expect("1 is always non-zero."),
            current_page: None,
        }
    }
}

impl<T> PageIterator<'_, '_, T>
where
    for<'deserialize> T: Deserialize<'deserialize>,
{
    /// Return the next item in the iterator, fetching a new page if necessary.
    pub async fn next(&mut self) -> Option<reqwest::Result<T>> {
        if let Some(item) = self.current_page.as_mut().and_then(Iterator::next) {
            return Some(Ok(item));
        }

        let mut next_page = match self
            .pager
            .page(self.page_no)
            .await
            .inspect_err(|error| error!("{error}"))
        {
            Ok(next_page) => next_page.into_iter(),
            Err(error) => return Some(Err(error)),
        };

        let item = next_page.next()?;
        self.current_page.replace(next_page);
        self.page_no = self.page_no.saturating_add(1);
        Some(Ok(item))
    }
}

impl<'session, 'path, T> From<Pager<'session, 'path>> for PageIterator<'session, 'path, T> {
    fn from(pager: Pager<'session, 'path>) -> Self {
        Self::new(pager)
    }
}
