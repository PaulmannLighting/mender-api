use std::num::NonZero;

use serde::Deserialize;

use crate::Session;

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
    /// Return the page size.
    pub const fn page_size(&self) -> NonZero<usize> {
        self.page_size
    }

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
