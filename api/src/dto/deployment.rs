//! Deployment-related data structures and types.

pub use self::kind::Kind;
pub use self::status::Status;

mod kind;
pub mod list;
pub mod new;
pub mod put;
mod status;
