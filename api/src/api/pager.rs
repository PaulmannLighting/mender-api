//! Devices management API.

use std::marker::PhantomData;
use std::num::NonZero;
use std::vec::IntoIter;

use log::error;
use serde::Deserialize;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::session::Session;

/// Generic pager.
#[derive(Debug, Clone)]
pub struct Pager<'session, 'path, T> {
    session: &'session Session,
    path: &'path str,
    page_size: NonZero<usize>,
    phantom_data: PhantomData<T>,
}

impl<'session, 'path, T> Pager<'session, 'path, T> {
    /// Create a new pager.
    #[must_use]
    pub const fn new(session: &'session Session, path: &'path str) -> Self {
        Self::new_with_page_size(session, path, DEFAULT_PAGE_SIZE)
    }

    /// Create a new pager.
    #[must_use]
    pub const fn new_with_page_size(
        session: &'session Session,
        path: &'path str,
        page_size: NonZero<usize>,
    ) -> Self {
        Self {
            session,
            path,
            page_size,
            phantom_data: PhantomData,
        }
    }
}

impl<T> Pager<'_, '_, T>
where
    for<'a> T: Deserialize<'a>,
{
    /// Return the given page.
    pub async fn page(&self, page_no: NonZero<usize>) -> reqwest::Result<Vec<T>> {
        let mut url = self.session.url(self.path);
        url.set_query(Some(&format!("per_page={}&page={page_no}", self.page_size)));
        self.session
            .client()
            .get(url)
            .bearer_auth(self.session.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
    /// Iterate over all pages,
    pub async fn iter(&self) -> reqwest::Result<Vec<T>> {
        let mut devices = Vec::new();

        for page_no in (1..).filter_map(NonZero::new) {
            let page = self.page(page_no).await?;

            if page.len() < self.page_size.get() {
                devices.extend(page);
                break;
            } else {
                devices.extend(page);
            }
        }

        Ok(devices)
    }
}

/// Iterator for paginated results.
#[derive(Debug, Clone)]
pub struct PageIterator<'session, 'path, T> {
    pager: Pager<'session, 'path, T>,
    page_no: NonZero<usize>,
    current_page: Option<IntoIter<T>>,
}

impl<'session, 'path, T> PageIterator<'session, 'path, T> {
    /// Create a new page iterator with the given page size.
    #[must_use]
    pub(crate) const fn new(pager: Pager<'session, 'path, T>) -> Self {
        Self {
            pager,
            page_no: NonZero::new(1).expect("1 is always non-zero."),
            current_page: None,
        }
    }
}

impl<T> PageIterator<'_, '_, T>
where
    for<'a> T: Deserialize<'a>,
{
    /// Return the next item in the iterator, fetching a new page if necessary.
    pub async fn next(&mut self) -> Option<T> {
        if let Some(item) = self.current_page.as_mut().and_then(Iterator::next) {
            return Some(item);
        }

        let mut next_page = self
            .pager
            .page(self.page_no)
            .await
            .inspect_err(|error| error!("{error}"))
            .ok()?
            .into_iter();

        let item = next_page.next()?;
        self.current_page.replace(next_page);
        self.page_no = self.page_no.saturating_add(1);
        Some(item)
    }
}

impl<'session, 'path, T> From<Pager<'session, 'path, T>> for PageIterator<'session, 'path, T> {
    fn from(pager: Pager<'session, 'path, T>) -> Self {
        Self::new(pager)
    }
}

#[macro_export]
macro_rules! async_iter {
    ($pattern:pat in $iter:expr => $body:block) => {
        {
            let mut __async_iterator = $iter;
            while let Some($pattern) = __async_iterator.next().await $body
        }
    }
}
