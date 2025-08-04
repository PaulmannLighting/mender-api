use crate::api::deployments::Deployments;
use crate::api::devauth::Devauth;
use crate::api::endpoint::Endpoint;
use crate::api::inventory::Inventory;
use crate::api::user_adm::UserAdm;

/// Management API interface for Mender server.
pub trait Management {
    /// Return the device authentication API interface.
    fn devauth(&self) -> impl Devauth;

    /// Return the inventory API interface.
    fn inventory(&self) -> impl Inventory;

    /// Return the user administration API interface.
    fn useradm(&self) -> impl UserAdm;

    /// Return the deployments API interface.
    fn deployments(&self) -> impl Deployments;
}
impl Management for Endpoint<'_> {
    fn devauth(&self) -> impl Devauth {
        Self::new(self.server, self.path.join("devauth"))
    }

    fn inventory(&self) -> impl Inventory {
        Self::new(self.server, self.path.join("inventory"))
    }

    fn useradm(&self) -> impl UserAdm {
        Self::new(self.server, self.path.join("useradm"))
    }

    fn deployments(&self) -> impl Deployments {
        Self::new(self.server, self.path.join("deployments"))
    }
}
