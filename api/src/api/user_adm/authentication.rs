use serde::Serialize;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Request<'username, 'password> {
    username: &'username str,
    password: &'password str,
}

impl<'username, 'password> Request<'username, 'password> {
    /// Creates a new `Authentication` instance with the provided username and password.
    pub const fn new(username: &'username str, password: &'password str) -> Self {
        Self { username, password }
    }
}

pub struct Response {
    pub token: String,
}
