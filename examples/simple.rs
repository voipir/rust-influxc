//!
//! Testing Sandbox
//!
use influxc::Client;
use influxc::FileBacklog;

use influxc::Record;
use influxc::Precision;
use influxc::Credentials;
use influxc::InfluxError;

use flexi_logger as logger;
use flexi_logger::Logger;

use std::time::Duration;
use std::thread::sleep;


fn run() -> Result<(), InfluxError>
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
