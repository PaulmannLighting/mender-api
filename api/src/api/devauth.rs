use crate::api::endpoint::Endpoint;

/// Device authentication API interface for Mender server.
pub trait Devauth {
    /// Return the devices API interface.
    fn devices(&self);
}

impl Devauth for Endpoint<'_> {
    fn devices(&self) {
        todo!()
    }
}
