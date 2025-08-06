use crate::Api;
use crate::api::session::Session;

const PATH: &str = "/api/management/v1/useradm/auth/login";

/// User login API.
pub trait Login {
    /// Log in to the Mender server with the given username and password.
    ///
    /// # Returns
    ///
    /// Returns a future that resolves to a [`Session`] object on success.
    ///
    /// # Errors
    ///
    /// If the login fails, the future will resolve to a [`reqwest::Error`].
    fn login<U, P>(
        self,
        user_name: U,
        password: P,
    ) -> impl Future<Output = reqwest::Result<Session>> + Send
    where
        U: AsRef<str> + Send,
        P: AsRef<str> + Send;
}

impl Login for Api {
    async fn login<U, P>(self, user_name: U, password: P) -> reqwest::Result<Session>
    where
        U: AsRef<str> + Send,
        P: AsRef<str> + Send,
    {
        self.client
            .post({
                let mut url = self.base_url.clone();
                url.set_path(PATH);
                url
            })
            .basic_auth(user_name.as_ref(), Some(password.as_ref()))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(|bearer_token| Session::new(self.client, self.base_url, bearer_token))
    }
}
