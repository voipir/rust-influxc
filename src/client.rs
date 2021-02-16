//!
//! Client Connection and Interface to Database
//!
use crate::Credentials;
use crate::Measurement;

use crate::Backlog;

use crate::InfluxError;
use crate::InfluxResult;

use crate::Record;
use crate::Precision;

use crate::ReqwClient;


#[derive(Deserialize)]
pub struct ResponseError
{
    error: String
}


#[derive(Debug)]
pub struct Client<'c>
{
    client: ReqwClient,

    backlog: Option<&'c mut dyn Backlog>,

    target_url:   String,
    target_db:    String,
    target_creds: Credentials,
}


impl<'c> Client<'c>
{
    pub fn new(url: String, db: String, creds: Credentials) -> InfluxResult<Self>
    {
        let client = ReqwClient::builder()
            .build()?;

        // TODO ping database

        Ok(Self {
            client,
            backlog: None,

            target_url:   url,
            target_db:    db,
            target_creds: creds,
        })
    }

    pub fn backlog(mut self, backlog: &'c mut dyn Backlog) -> Self
    {
        self.backlog = Some(backlog); self
    }

    pub fn write(&mut self, record: &Record) -> InfluxResult<()>
    {
        self.write_backlog()?;
        self.write_record(&record)?;

        Ok(())
    }
}


impl<'c> Client<'c>
{
    fn write_backlog(&mut self) -> InfluxResult<()>
    {
        let records = if let Some(blg) = &mut self.backlog {
            blg.read_pending()?
        } else {
            Vec::new()
        };

        for record in records
        {
            info!("Found {} backlogged entries, proceeding to commit", records.len());

            if let Err(e) = self.write_record(&record) {
                return Err(InfluxError::Error(format!("Unable to commit backlogged record: {}", e)));
            }
            else
            {
                let result = self.backlog.as_mut()
                    .unwrap()
                    .truncate_pending();

                if let Err(e) = result
                {
                    let msg = format!("Failed to eliminate/truncate record from backlog: {}", e);
                    error!("{}", msg);
                    panic!("{}", msg);
                }
                else {
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn write_record(&self, record: &Record) -> InfluxResult<()>
    {
        let url = format!("{}/write", self.target_url);

        // TODO handle in one query, if necessary grouping by query parameters. For this we need to figure out how to
        // handle precision and retention policy of the individual measurements. If moved out of measurement intself
        // it would enable for bulk queries, but force persistent file backlog storage to store them separately from the
        // serialized measurement.
        for point in points.iter()
        {
            let line = point.to_line();

            debug!("Inserting to influxdb: {}", line);

            let mut params = vec![
                ("db",        self.target_db.to_owned()),
                // ("precision", point.precision.to_string()),
            ];

            // if let Some(policy) = &point.retpolicy {
            //     params.push(("rp", policy.to_owned()));
            // }

            let result = self.client.post(&url)
                .basic_auth(&self.target_creds.user, Some(&self.target_creds.passwd))
                .query(&params)
                .body(line)
                .send()?;

            if result.status().is_success() {
                continue;
            } else if result.status().is_client_error() || result.status().is_server_error()
            {
                let error: ResponseError = result.json()?;
                return Err(format!("Could not commit measurement to db: {}", error.error).into());
            }
        }

        Ok(())
    }
}
