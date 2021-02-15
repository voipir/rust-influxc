//!
//! Testing Sandbox
//!
use influxdb::Client;
use influxdb::FileBacklog;

use influxdb::Precision;
use influxdb::Credentials;
use influxdb::Measurement;
use influxdb::InfluxResult;

use chrono::Utc as ChronoUtc;

use flexi_logger as logger;
use flexi_logger::Logger;

use std::time::Duration;
use std::thread::sleep;


fn run() -> InfluxResult<()>
{
    let creds   = Credentials::new("testuser".into(), "testpasswd".into());
    let backlog = FileBacklog::new("./ignore/backlog.json".into())?;

    let mut client = Client::new("http://127.0.0.1:8044".into(), "test".into(), creds)?
        .backlog(backlog);

    loop
    {
        let point = Measurement::with_timestamp("test", ChronoUtc::now(), Precision::Milliseconds)
            .add_tag("type", "main")
            .add_field("asd", 123.into());

        client.write_one(point)?;

        sleep(Duration::from_secs(1));
    }
}


fn main()
{
    Logger::with_env_or_str("info")
        .format(logger::opt_format)
        .start()
        .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

    if let Err(e) = run() {
        println!("{}", e)
    }
}
