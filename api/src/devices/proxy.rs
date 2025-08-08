use uuid::Uuid;

use super::PATH;
use crate::dto::DeviceGroup;
use crate::session::Session;

/// Proxy object to manage a device on the Mender server.
#[derive(Clone, Debug)]
pub struct Proxy<'session> {
    session: &'session Session,
    id: Uuid,
}

impl<'session> Proxy<'session> {
    /// Create a new `Proxy` for the specified device ID.
    pub(crate) const fn new(session: &'session Session, id: Uuid) -> Self {
        Self { session, id }
    }
}

impl Proxy<'_> {
    /// Return the status of the device.
    pub async fn status(&self) -> reqwest::Result<String> {
        let url = self
            .session
            .format_url(format!("{PATH}/{}/status", self.id), None);
        let response = self
            .session
            .client()
            .get(url)
            .bearer_auth(self.session.bearer_token())
            .send()
            .await?
            .error_for_status()?;
        let status: String = response.json().await?;
        Ok(status)
    }

    /// Add the device to the specified group.
    pub async fn add_to_group<T>(&self, group_name: T) -> reqwest::Result<()>
    where
        T: AsRef<str>,
    {
        let url = self
            .session
            .format_url(format!("{PATH}/{}/group", self.id), None);
        self.session
            .client()
            .post(url)
            .bearer_auth(self.session.bearer_token())
            .json(&DeviceGroup::new(group_name.as_ref()))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}
