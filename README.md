# InfluxDB Client Library

## About this crate

### What this crate provides

* Support for InfluxDB 2.x.
* Backlog storage of Record's on failure to commit due to connectivity or configuration issues.
* Build-in compression of requests.

### What it does not provide

* Support for InfluxDB 1.x

### What is on the roadmap

* Support for sending, processing responses to queries.
* Support for mapping native types to query response data like sqlx.
* Support for async/await as a feature.
* Reduction of dependencies by switching the underlying reqwest library with hyper.

## Basic Usage

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
