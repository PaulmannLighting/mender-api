//! Extensions trait to handle joining paths with Mender URLs.

use std::path::Path;

use reqwest::Url;

/// Extension trait for `Url` to handle joining paths with Mender URLs.
pub trait JoinPath {
    /// Joins the base URL with a path and returns a new `Url`.
    ///
    /// # Panics
    ///
    /// If the resulting path is not valid UTF-8, this will panic.
    fn join_path<T>(&mut self, path: T)
    where
        T: AsRef<Path>;

    /// Joins the base URL with a path and returns `Self`.
    fn with_joined_path<T>(mut self, path: T) -> Self
    where
        Self: Sized,
        T: AsRef<Path>,
    {
        self.join_path(path);
        self
    }
}

impl JoinPath for Url {
    fn join_path<T>(&mut self, path: T)
    where
        T: AsRef<Path>,
    {
        self.set_path(
            Path::new(self.path())
                .join(path)
                .as_os_str()
                .to_str()
                .expect("Path should be valid UTF-8."),
        );
    }
}
