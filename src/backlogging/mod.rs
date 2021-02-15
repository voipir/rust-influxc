//!
//! Data Constistency Guarantee Methods.
//!
//! These are designed to provide the same interface than the client, wrapping around it and providing the same
//! interface `ClientTrait`.
//!
mod file;

pub use file::FileBacklog;

use crate::Measurement;
use crate::InfluxResult;


/// API definition that any backlog service needs to abide by so the Client can use it.
pub trait Backlog
{
    /// Return any pending measurement that sits in backlog and requires to be commited.
    fn read_pending(&mut self) -> InfluxResult<Vec<Measurement>>;

    /// Write measurements that could not be commited, so they get written into backlog for future processing.
    fn write_pending(&mut self, points: &[Measurement]) -> InfluxResult<()>;

    /// Empty backlog from pending measurements. This gets called once all pending measurements have been
    /// successfully commited.
    fn truncate_pending(&mut self) -> InfluxResult<()>;
}
