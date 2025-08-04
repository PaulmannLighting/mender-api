use crate::api::endpoint::Endpoint;

/// Inventory API.
pub trait Inventory {
    /// Return the devices API interface.
    fn devices(&self) -> reqwest::Url;

    /// Return the groups API interface.
    fn groups(&self) -> reqwest::Url;
}
impl Inventory for Endpoint<'_> {
    fn devices(&self) -> reqwest::Url {
        todo!()
    }

    fn groups(&self) -> reqwest::Url {
        todo!()
    }
}
