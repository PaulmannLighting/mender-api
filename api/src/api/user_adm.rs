use serde_json::json;

use crate::api::endpoint::Endpoint;
use crate::util::JoinPath;

const LOGIN_ENDPOINT: &str = "auth/login";

/// User administration API.
pub trait UserAdm {
    async fn login(
        &self,
        user_name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<String, reqwest::Error>;
}

impl UserAdm for Endpoint<'_> {
    async fn login(
        &self,
        user_name: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<String, reqwest::Error> {
        let url = self.url().with_joined_path(LOGIN_ENDPOINT);
        let response = self
            .server
            .client
            .post(url)
            .basic_auth(user_name.as_ref(), Some(password.as_ref()))
            .json(&json!({}))
            .send()
            .await?
            .error_for_status()?;
        response.text().await
    }
}
