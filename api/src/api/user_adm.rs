mod authentication;

use crate::api::endpoint::Endpoint;
use crate::api::user_adm::authentication::Request;
use crate::util::JoinPath;

const LOGIN_ENDPOINT: &str = "auth/login";

/// User administration API.
pub trait UserAdm {
    async fn login(
        &self,
        user_name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Vec<u8>, reqwest::Error>;
}

impl UserAdm for Endpoint<'_> {
    async fn login(
        &self,
        user_name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Vec<u8>, reqwest::Error> {
        let authentication = Request::new(user_name.as_ref(), password.as_ref());
        let url = self.url().with_joined_path(LOGIN_ENDPOINT);
        let response = self
            .server
            .client
            .post(url)
            .json(&authentication)
            .send()
            .await?
            .error_for_status()?;

        response.bytes().await.map(|bytes| bytes.to_vec())
    }
}
