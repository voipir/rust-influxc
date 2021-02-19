//!
//! Client Connection and Interface to Database
//!
use crate::Record;
use crate::Backlog;
use crate::Credentials;

use crate::InfluxError;
use crate::InfluxResult;

use crate::ReqwUrl;
use crate::ReqwClient;
use crate::ReqwMethod;


#[derive(Deserialize)]
pub struct ResponseError
{
    error: String
}


#[derive(Debug)]
pub struct Client<B>
    where B: Backlog
{
    client: ReqwClient,

    backlog: Option<B>,

    target_url:   ReqwUrl,
    target_db:    String,
    target_creds: Credentials,
}


impl<B> Client<B>
    where B: Backlog
{
    pub fn new(url: String, db: String, creds: Credentials) -> InfluxResult<Self>
    {
        let client = ReqwClient::builder()
            .build()?;

        let url = match ReqwUrl::parse(&url)
        {
            Ok(url) => { url }
            Err(e)  => { return Err(format!("Failed to parse URL: {} due to {}", url, e).into()) }
        };

        // TODO ping database

        Ok(Self {
            client,
            backlog: None,

            target_url:   url,
            target_db:    db,
            target_creds: creds,
        })
    }

    pub fn backlog(mut self, backlog: B) -> Self
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


impl<B> Client<B>
    where B: Backlog
{
    fn write_backlog(&mut self) -> InfluxResult<()>
    {
        let records = if let Some(blg) = &mut self.backlog {
            blg.read_pending()?
        } else {
            Vec::new()
        };

        for record in records.iter()
        {
            info!("Found {} backlogged entries, proceeding to commit", records.len());

            if let Err(e) = self.write_record(&record) {
                return Err(InfluxError::Error(format!("Unable to commit backlogged record: {}", e)));
            }
            else
            {
                let result = self.backlog.as_mut()
                    .unwrap()
                    .truncate_pending(&record);

                if let Err(e) = result {
                    let msg = format!("Failed to eliminate/truncate record from backlog: {}", e);
                    error!("{}", msg);
                    panic!("{}", msg);
                } else {
                    return Ok(());
                }
            }
        }

        Ok(())
    }

    fn write_record(&self, record: &Record) -> InfluxResult<()>
    {
        let mut url = self.target_url.clone();

        url.set_path("/api/v2/write");

        let mut builder = self.client.request(ReqwMethod::POST, url)
            .query(&[
                ("org",       &record.org),
                ("bucket",    &record.bucket),
                ("precision", &record.precision.to_string()),
            ]);

        match &self.target_creds
        {
            Credentials::Basic{user, passwd} => {
                builder = builder.header("Authorization", format!("Basic {} {}", user, passwd));
            }

            Credentials::Token{token} => {
                builder = builder.header("Authorization", format!("Token {}", token));
            }
        }

        let reply = builder.body(record.to_line_buffer())
            .send()?;

        if reply.status().is_success() {
            Ok(())
        } else if reply.status().is_client_error() || reply.status().is_server_error()
        {
            let error: ResponseError = reply.json()?;
            Err(format!("Could not commit record to db: {}", error.error).into())
        } else {
            Err(format!("Something else happened. Status: {:?}", reply.status()).into())
        }
    }
}
