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
    done: bool,
    phantom: PhantomData<T>,
}

impl<'session, 'path, T> Pages<'session, 'path, T> {
    /// Create a new pages iterator with the given page size.
    #[must_use]
    pub(crate) const fn new(pager: Pager<'session, 'path>) -> Self {
        Self {
            pager,
            page_no: NonZero::new(1).expect("1 is always non-zero."),
            done: false,
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
        if self.done {
            return None;
        }

        let page_no = self.page_no;
        self.page_no = self.page_no.saturating_add(1);

        match self
            .pager
            .page(page_no)
            .await
            .inspect_err(|error| error!("{error}"))
        {
            Ok(page) => {
                if page.is_empty() {
                    self.done = true;
                    return None;
                }

                if page.len() < self.pager.page_size().get() {
                    self.done = true;
                }

                Some(Ok(page))
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
