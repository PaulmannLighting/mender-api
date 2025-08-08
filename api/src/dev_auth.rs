use std::num::NonZero;

use serde::Serialize;
use uuid::Uuid;

use crate::dto::{DevAuthDevice, Status};
use crate::pager::{DEFAULT_PAGE_SIZE, PageIterator, Pager};
use crate::session::Session;

const PATH: &str = "/api/management/v2/devauth/devices";

/// API to the device authentication (`DevAuth`) service.
pub trait DevAuth {
    /// List devices with the specified page size.
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, DevAuthDevice>;

    /// Set the status of the device with the specified ID.
    fn set_status(
        &self,
        id: Uuid,
        auth_id: Uuid,
        status: Status,
    ) -> impl Future<Output = reqwest::Result<String>> + Send;
}

impl DevAuth for Session {
    fn list(&self, page_size: Option<NonZero<usize>>) -> PageIterator<'_, '_, DevAuthDevice> {
        Pager::new(self, PATH, page_size.unwrap_or(DEFAULT_PAGE_SIZE)).into()
    }

    async fn set_status(&self, id: Uuid, auth_id: Uuid, status: Status) -> reqwest::Result<String> {
        self.client()
            .put(self.format_url(format!("{PATH}/{id}/auth/{auth_id}/status"), None))
            .bearer_auth(self.bearer_token())
            .json(&SetStatusRequest { status })
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize)]
struct SetStatusRequest {
    status: Status,
}
