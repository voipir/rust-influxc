//!
//! InfluxDB Client Library
//!
#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use serde_json as json;
use serde_json::error::Error as JsonError;

use reqwest::Error            as ReqwError;
use reqwest::blocking::Client as ReqwClient;

type ChronoDateTime = chrono::DateTime<chrono::Utc>;

//
// Internals
//
mod error;
mod value;
mod client;
mod precision;
mod backlogging;
mod credentials;
mod measurement;

//
// Exports
//
pub use error::InfluxError;
pub use error::InfluxResult;

pub use value::Value;

pub use client::Client;

pub use precision::Precision;

pub use backlogging::Backlog;
pub use backlogging::FileBacklog;

pub use credentials::Credentials;

pub use measurement::Measurement;
