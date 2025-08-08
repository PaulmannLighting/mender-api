use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::{Identity, Status};

/// Authentification data set for a device in the Mender server.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthentificationSet {
    id: Uuid,
    #[serde(rename = "identity_data")]
    identity: Identity,
    pubkey: String,
    ts: String,
    status: Status,
}

impl AuthentificationSet {
    /// Returns the ID of the authentification set.
    #[must_use]
    pub const fn id(&self) -> Uuid {
        self.id
    }

    /// Returns the identity associated with the authentification set.
    #[must_use]
    pub const fn identity(&self) -> &Identity {
        &self.identity
    }

    /// Returns the public key of the authentification set.
    #[must_use]
    pub fn pubkey(&self) -> &str {
        &self.pubkey
    }

    /// Returns the timestamp of the authentification set.
    #[must_use]
    pub fn ts(&self) -> &str {
        &self.ts
    }

    /// Returns the status of the authentification set.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }
}
