//!
//! InfluxDB Client Library
//!
#[macro_use] extern crate log;
#[macro_use] extern crate serde;
#[macro_use] extern crate error_chain;

use serde_json as json;
use serde_json::error::Error as JsonError;

use reqwest::Error    as ReqwError;
use reqwest::UrlError as ReqwUrlError;
use reqwest::Client   as ReqwClient;

type ChronoDateTime = chrono::DateTime<chrono::Utc>;

//
// Internals
//
mod error;
mod value;
mod client;
mod persist;
mod precision;
mod credentials;
mod measurement;

//
// Exports
//
pub use self::error::InfluxError;
pub use self::error::InfluxErrorKind;
pub use self::error::InfluxResult;
pub use self::error::InfluxResultExt;

pub use self::value::Value;

pub use self::client::Client;
pub use self::client::ClientTrait;

pub use self::persist::FileBackloggedClient;

pub use self::precision::Precision;

pub use self::credentials::Credentials;

pub use self::measurement::Measurement;
