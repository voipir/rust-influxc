//!
//! Testing Sandbox
//!
use influxc::Client;
use influxc::FileBacklog;

use influxc::Record;
use influxc::Precision;
use influxc::Credentials;
use influxc::InfluxError;

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
            .field("temp", 123)
            .field("brightness", 500);

        rec.measurement("sensor2")
            .tag("floor", "second")
            .tag("exposure", "east")
            .field("temp", 321)
            .field("brightness", 999);

        if let Err(e) = client.write(&rec) {
            eprintln!("{}", e);
        }

        sleep(Duration::from_secs(1));
    }
}
