use std::marker::PhantomData;
use std::num::NonZero;

use log::error;
use serde::Deserialize;

use crate::Pager;

/// Iterator over pages of results.
#[derive(Debug, Clone)]
pub struct Pages<'session, 'path, T> {
    pager: Pager<'session, 'path>,
    page_no: NonZero<usize>,
    phantom: PhantomData<T>,
}

impl<'session, 'path, T> Pages<'session, 'path, T> {
    /// Create a new pages iterator with the given page size.
    #[must_use]
    pub(crate) const fn new(pager: Pager<'session, 'path>) -> Self {
        Self {
            pager,
            page_no: NonZero::new(1).expect("1 is always non-zero."),
            phantom: PhantomData,
        }
    }
}

impl<T> Pages<'_, '_, T>
where
    for<'deserialize> T: Deserialize<'deserialize> + Send + Sync,
{
    /// Return the next page.
    pub async fn next(&mut self) -> Option<reqwest::Result<Vec<T>>> {
        let page_no = self.page_no;
        self.page_no = self.page_no.saturating_add(1);

        match self
            .pager
            .page(page_no)
            .await
            .inspect_err(|error| error!("{error}"))
        {
            Ok(next_page) => {
                if next_page.is_empty() {
                    return None;
                }

                Some(Ok(next_page))
            }
            Err(error) => Some(Err(error)),
        }
    }
}

impl<'session, 'path, T> From<Pager<'session, 'path>> for Pages<'session, 'path, T> {
    fn from(pager: Pager<'session, 'path>) -> Self {
        Self::new(pager)
    }
}
