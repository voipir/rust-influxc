/*!
InfluxDB Client Library

# About this crate

### What this crate provides

* Support for InfluxDB 2.x.
* Backlog storage of Record's on failure to commit due to connectivity or configuration issues.
* Build-in compression of requests.

### What it does not provide

* Support for InfluxDB 1.x

### What is on the roadmap

* Support for async/await as a feature.
* Reduction of dependencies by switching the underlying reqwest library with hyper.

# Basic Usage

```rust
use influxdb::Client;
use influxdb::FileBacklog;

use influxdb::Record;
use influxdb::Precision;
use influxdb::Credentials;
use influxdb::InfluxError;

use std::time::Duration;
use std::thread::sleep;

fn main() -> Result<(), InfluxError>
{
    let creds   = Credentials::from_basic("testuser", "testpasswd");
    let backlog = FileBacklog::new("./ignore/backlog")?;

    let mut client = Client::build("http://127.0.0.1:8086".into(), creds)
        .backlog(backlog)
        .finish()?;

    let mut rec = Record::new("org", "bucket")
        .precision(Precision::Milliseconds);

    loop
    {
        rec.measurement("sensor1")
            .tag("floor", "second")
            .tag("exposure", "west")
            .field("temp", 123.into())
            .field("brightness", 500.into());

        rec.measurement("sensor2")
            .tag("floor", "second")
            .tag("exposure", "east")
            .field("temp", 321.into())
            .field("brightness", 999.into());

        if let Err(e) = client.write(&rec) {
            eprintln!("{}", e);
        }

        sleep(Duration::from_secs(1));
    }
}
```
*/
#![warn(missing_docs)]

#![allow(clippy::new_without_default)]
#![allow(clippy::suspicious_else_formatting)]

// Imports
#[macro_use] extern crate log;
#[macro_use] extern crate serde;

use serde::Deserialize;

use flate2::GzBuilder   as FlateGzipBuilder;
use flate2::Compression as FlateLevel;

use serde_json as json;
use serde_json::error::Error as JsonError;

use base32 as b32;
use base64 as b64;

use reqwest::Url    as ReqwUrl;
use reqwest::Error  as ReqwError;
use reqwest::Method as ReqwMethod;

use reqwest::blocking::Client         as ReqwClient;
use reqwest::blocking::RequestBuilder as ReqwRequestBuilder;

type Utc      = chrono::Utc;
type DateTime = chrono::DateTime<chrono::Utc>;

// Internals/Exports
mod auth;
mod error;
mod value;
mod client;
mod record;
mod builder;
mod precision;
mod backlogging;
mod measurement;

use error::InfluxResult;
use error::InfluxErrorAnnotate;

use error::ApiDelayError;
use error::ApiGenericError;
use error::ApiOversizeError;
use error::ApiMalformationError;

pub use auth::Credentials;

pub use error::InfluxError;

pub use value::Value;

pub use client::Client;

pub use record::Record;

pub use builder::ClientBuilder;

pub use precision::Precision;

pub use backlogging::Backlog;
pub use backlogging::FileBacklog;
pub use backlogging::NoopBacklog;

pub use measurement::Measurement;
