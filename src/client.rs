//!
//! Client Connection and Interface to Database
//!
use crate::Credentials;
use crate::Measurement;
use crate::InfluxResult;
use crate::FileBackloggedClient;

use crate::ReqwClient;


pub trait ClientTrait
{
    fn write_one(&mut self, point: Measurement) -> InfluxResult<()>;
    fn write_many(&mut self, points: &[Measurement]) -> InfluxResult<()>;
}


#[derive(Deserialize)]
pub struct ResponseError
{
    error: String
}


#[derive(Debug)]
pub struct Client
{
    client: ReqwClient,

    target_url:   String,
    target_db:    String,
    target_creds: Credentials,

}


impl Client
{
    pub fn new(url: String, db: String, creds: Credentials) -> InfluxResult<Self>
    {
        let client = ReqwClient::builder()
            .build()?;

        // TODO ping database

        Ok(Self {
            client,
            target_url:   url,
            target_db:    db,
            target_creds: creds,
        })
    }

    pub fn into_file_backlogged(self, path: String) -> InfluxResult<FileBackloggedClient>
    {
        Ok(FileBackloggedClient::new(self, path)?)
    }
}


impl ClientTrait for Client
{
    fn write_one(&mut self, point: Measurement) -> InfluxResult<()>
    {
        self.write_many(&[point])
    }

    fn write_many(&mut self, points: &[Measurement]) -> InfluxResult<()>
    {
        let url = format!("{}/write", self.target_url);

        // TODO handle in one query, if necessary grouping by query parameters. For this we need to figure out how to
        // handle precision and retention policy of the individual measurements. If moved out of measurement intself
        // it would enable for bulk queries, but force persistent file backlog storage to store them separately from the
        // serialized measurement.
        for point in points.iter()
        {
            let line = point.to_line();

            let mut params = vec![
                ("db",        self.target_db.to_owned()),
                ("precision", point.precision.to_string()),
            ];

            if let Some(policy) = &point.retpolicy {
                params.push(("rp", policy.to_owned()));
            }

            let mut result = self.client.post(&url)
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
