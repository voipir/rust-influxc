//!
//! Testing Sandbox
//!
use influxdb::Client;
use influxdb::FileBacklog;

use influxdb::Record;
use influxdb::Precision;
use influxdb::Credentials;
use influxdb::InfluxResult;

use flexi_logger as logger;
use flexi_logger::Logger;

use std::time::Duration;
use std::thread::sleep;


fn run() -> InfluxResult<()>
{
    let creds   = Credentials::from_basic("testuser".into(), "testpasswd".into());
    let backlog = FileBacklog::new("./ignore/backlog.json")?;

    let mut client = Client::new("http://127.0.0.1:8086".into(), "test".into(), creds)?
        .backlog(backlog);

    loop
    {
        let mut record = Record::new("org", "bucket", Precision::Seconds);

        record.measurement("sensor1")
            .tag("floor", "second")
            .tag("exposure", "west")
            .field("temp", 123.into())
            .field("brightness", 999.into());

        record.measurement("sensor2")
            .tag("floor", "second")
            .tag("exposure", "east")
            .field("temp", 321.into())
            .field("brightness", 999.into());

        client.write(&record)?;

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
