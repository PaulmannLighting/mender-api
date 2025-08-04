use crate::api::endpoint::Endpoint;

/// Deployments API.
pub trait Deployments {
    fn deployments(&self);
}

impl Deployments for Endpoint<'_> {
    fn deployments(&self) {
        todo!()
    }
}
