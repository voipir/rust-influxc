//!
//! Data Constistency Guarantee Methods.
//!
//! These are designed to provide the same interface than the client, wrapping around it and providing the same
//! interface `ClientTrait`.
//!
mod file;

pub use file::FileBackloggedClient;
