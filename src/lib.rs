//!
//! InfluxDB Client Library
//!
#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use serde_json as json;
use serde_json::error::Error as JsonError;

use reqwest::Url                      as ReqwUrl;
use reqwest::Error                    as ReqwError;
use reqwest::Method                   as ReqwMethod;
use reqwest::blocking::Client         as ReqwClient;

type Utc      = chrono::Utc;
type DateTime = chrono::DateTime<chrono::Utc>;

//
// Internals
//
mod auth;
mod error;
mod value;
mod client;
mod record;
mod precision;
mod backlogging;
mod measurement;

//
// Exports
//
pub use auth::Credentials;

pub use error::InfluxError;
pub use error::InfluxResult;

pub use value::Value;

pub use client::Client;

pub use record::Record;

pub use precision::Precision;

pub use backlogging::Backlog;
pub use backlogging::FileBacklog;

pub use measurement::Measurement;
