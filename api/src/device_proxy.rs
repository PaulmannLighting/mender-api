use uuid::Uuid;

use crate::session::Session;

/// A proxy for a device in the Mender server.
pub struct DeviceProxy<'session> {
    session: &'session Session,
    id: Uuid,
}

impl<'session> DeviceProxy<'session> {
    /// Create a new `Proxy` for the specified device ID and session.
    #[must_use]
    pub(crate) const fn new(session: &'session Session, id: Uuid) -> Self {
        Self { session, id }
    }

    /// Return the session associated with this device proxy.
    #[must_use]
    pub(crate) const fn session(&self) -> &Session {
        self.session
    }

    /// Return the ID of the device.
    #[must_use]
    pub(crate) const fn id(&self) -> Uuid {
        self.id
    }
}
