use crate::api::session::Session;

/// Device authentication API interface for Mender server.
pub trait Devauth {
    /// Return the devices API interface.
    fn devices(&self);
}

impl Devauth for Session {
    fn devices(&self) {
        todo!()
    }
}
