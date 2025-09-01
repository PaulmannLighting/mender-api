//! Paging of multi-page result listings.

use std::num::NonZero;
use std::vec::IntoIter;

use log::error;
use serde::Deserialize;

use crate::session::Session;

pub const DEFAULT_PAGE_SIZE: NonZero<usize> =
    NonZero::new(500).expect("Default page should be be non-zero.");

/// Generic pager.
#[derive(Debug, Clone)]
pub struct Pager<'session, 'path> {
    session: &'session Session,
    path: &'path str,
    page_size: NonZero<usize>,
}

impl<'session, 'path> Pager<'session, 'path> {
    /// Create a new pager.
    #[must_use]
    pub const fn new(
        session: &'session Session,
        path: &'path str,
        page_size: NonZero<usize>,
    ) -> Self {
        Self {
            session,
            path,
            page_size,
        }
    }
}

impl Pager<'_, '_> {
    /// Return the given page.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if the request fails or the response cannot be deserialized.
    pub async fn page<T>(&self, page_no: NonZero<usize>) -> reqwest::Result<Vec<T>>
    where
        for<'deserialize> T: Deserialize<'deserialize>,
    {
        self.session
            .client()
            .get(self.session.format_url(
                self.path,
                format!("per_page={}&page={page_no}", self.page_size),
            ))
            .bearer_auth(self.session.bearer_token())
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }

    /// Iterate over all pages and collect the results into a `Vec`.
    ///
    /// # Errors
    ///
    /// Returns a [`reqwest::Error`] if any of the page requests fail.
    pub async fn collect<T>(&self) -> reqwest::Result<Vec<T>>
    where
        for<'deserialize> T: Deserialize<'deserialize>,
    {
        let mut devices = Vec::new();

        for page_no in (1..).filter_map(NonZero::new) {
            let page = self.page(page_no).await?;
            let page_size = page.len();
            devices.extend(page);

            if page_size < self.page_size.get() {
                break;
            }
        }

        Ok(devices)
    }
}

/// Iterator over pages of results.
#[derive(Debug, Clone)]
pub struct Pages<'session, 'path> {
    pager: Pager<'session, 'path>,
    page_no: NonZero<usize>,
}

impl<'session, 'path> Pages<'session, 'path> {
    /// Create a new pages iterator with the given page size.
    #[must_use]
    pub(crate) const fn new(pager: Pager<'session, 'path>) -> Self {
        Self {
            pager,
            page_no: NonZero::new(1).expect("1 is always non-zero."),
        }
    }
}

impl Pages<'_, '_> {
    /// Return the next page.
    pub async fn next<T>(&mut self) -> Option<reqwest::Result<Vec<T>>>
    where
        for<'deserialize> T: Deserialize<'deserialize>,
    {
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

impl<'session, 'path> From<Pager<'session, 'path>> for Pages<'session, 'path> {
    fn from(pager: Pager<'session, 'path>) -> Self {
        Self::new(pager)
    }
}

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
