//! Devices management API.

use std::marker::PhantomData;
use std::num::NonZero;

use log::debug;
use serde::Deserialize;

use crate::api::DEFAULT_PAGE_SIZE;
use crate::api::session::Session;

/// Generic pager.
pub struct Pager<'session, 'path, T> {
    session: &'session Session,
    path: &'path str,
    phantom_data: PhantomData<T>,
}

impl<'session, 'path, T> Pager<'session, 'path, T> {
    /// Create a new pager.
    #[must_use]
    pub const fn new(session: &'session Session, path: &'path str) -> Self {
        Self {
            session,
            path,
            phantom_data: PhantomData,
        }
    }
}

impl<T> Pager<'_, '_, T>
where
    for<'a> T: Deserialize<'a>,
{
    /// Return the given page.
    pub async fn page(
        &self,
        page_size: NonZero<usize>,
        page_no: NonZero<usize>,
    ) -> reqwest::Result<Vec<T>> {
        let mut url = self.session.url(self.path);
        url.set_query(Some(&format!("per_page={page_size}&page={page_no}")));
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
    /// Iterate over all pages with the specified page size,
    pub async fn iter_sized(&self, page_size: NonZero<usize>) -> reqwest::Result<Vec<T>> {
        let mut devices = Vec::new();

        for page_no in (1..).filter_map(NonZero::new) {
            debug!("Fetching devices page #{page_no} with size {page_size}");
            let page = self.page(page_size, page_no).await?;

            if page.len() < page_size.get() {
                devices.extend(page);
                break;
            } else {
                devices.extend(page);
            }
        }

        Ok(devices)
    }

    /// Iterate over all pages with the default page size.
    pub async fn iter(&self) -> reqwest::Result<Vec<T>> {
        self.iter_sized(DEFAULT_PAGE_SIZE).await
    }
}
