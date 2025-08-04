use crate::api::session::Session;

/// Deployments API.
pub trait Deployments {
    fn deployments(&self);
}

impl Deployments for Session {
    fn deployments(&self) {
        todo!()
    }
}
