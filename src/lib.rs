//!
//! InfluxDB Client Library
//!
#![allow(clippy::new_without_default)]
#![allow(clippy::suspicious_else_formatting)]

#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use serde::Deserialize;

use serde_json as json;
use serde_json::error::Error as JsonError;

use base32 as b32;
use base64 as b64;

// const B32RFC4648 = b32::Alphabet::RFC4648 {padding: false};

use reqwest::Url    as ReqwUrl;
use reqwest::Error  as ReqwError;
use reqwest::Method as ReqwMethod;

use reqwest::blocking::Client         as ReqwClient;
use reqwest::blocking::RequestBuilder as ReqwRequestBuilder;

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
mod builder;
mod precision;
mod backlogging;
mod measurement;

use error::ApiDelayError;
use error::ApiGenericError;
use error::ApiOversizeError;
use error::ApiMalformationError;

//
// Exports
//
pub use auth::Credentials;

pub use error::InfluxError;
pub use error::InfluxErrorAnnotate;
pub use error::InfluxResult;

pub use value::Value;

pub use client::Client;

pub use record::Record;

pub use builder::ClientBuilder;

pub use precision::Precision;

pub use backlogging::Backlog;
pub use backlogging::FileBacklog;
pub use backlogging::NoopBacklog;

pub use measurement::Measurement;
