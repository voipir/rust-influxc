//!
//! Client Connection and Interface to Database
//!
use crate::Record;
use crate::Backlog;
use crate::Credentials;

use crate::InfluxError;
use crate::InfluxResult;

use crate::ApiDelayError;
use crate::ApiGenericError;
use crate::ApiOversizeError;
use crate::ApiMalformationError;

use crate::b64;

use crate::ReqwUrl;
use crate::ReqwClient;
use crate::ReqwMethod;
use crate::ReqwRequestBuilder;


#[derive(Debug)]
pub struct Client<B>
    where B: Backlog
{
    client: ReqwClient,

    backlog: Option<B>,

    url:   ReqwUrl,
    creds: Credentials,
}


impl<B> Client<B>
    where B: Backlog
{
    pub fn new(url: String, creds: Credentials) -> InfluxResult<Self>
    {
        let client = ReqwClient::builder()
            .build()?;

        let url = match ReqwUrl::parse(&url)
        {
            Ok(url) => { url }
            Err(e)  => { return Err(format!("Failed to parse URL: {} due to {}", url, e).into()) }
        };

        let mut this = Self {client, url, creds, backlog: None};

        this.authenticate()?;

        Ok(this)
    }

    pub fn backlog(mut self, backlog: B) -> Self
    {
        self.backlog = Some(backlog); self
    }

    pub fn write(&mut self, record: &Record) -> InfluxResult<()>
    {
        if let Err(e) = self.write_backlog()
        {
            if let Some(ref mut backlog) = self.backlog {
                backlog.write_pending(&record)?;
            }

            Err(e)
        }
        else
        {
            let result = self.write_record(&record);

            if result.is_err() {
                if let Some(ref mut backlog) = self.backlog {
                    backlog.write_pending(&record)?;
                }
            }

            result
        }
    }

    pub fn flush(&mut self) -> InfluxResult<()>
    {
        self.write_backlog()
    }
}


/// Private interface
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
            info!("Found {} backlogged entries, attempting to commit", records.len());

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
        let mut url = self.url.clone();

        url.set_path("/api/v2/write");

        let mut builder = self.client.request(ReqwMethod::POST, url);

        builder = record.to_write_request(builder);
        builder = self.inject_credentials(builder)?;

        debug!("Request: {:#?}", builder);

        let reply = builder.send()?;

        match reply.status().as_u16()
        {
            204 => { info!("Written: {}", record); Ok(()) }

            400 => { Err(InfluxError::WriteMalformed(reply.json::<ApiMalformationError>()?)) }
            401 => { Err(InfluxError::WriteUnauthorized(reply.json::<ApiGenericError>()?)) }
            403 => { Err(InfluxError::WriteUnauthenticated(reply.json::<ApiGenericError>()?)) }
            413 => { Err(InfluxError::WriteOversized(reply.json::<ApiOversizeError>()?)) }
            429 => { Err(InfluxError::WriteOverquota(reply.json::<ApiDelayError>()?)) }
            503 => { Err(InfluxError::WriteUnready(reply.json::<ApiDelayError>()?)) }

            _   => { Err(InfluxError::WriteUnknown(reply.json::<ApiGenericError>()?)) }
        }
    }

    fn authenticate(&mut self) -> InfluxResult<()>
    {
        if let Credentials::Basic{ref user, ref passwd, cookie: None} = self.creds
        {
            let mut url = self.url.clone();

            url.set_path("/api/v2/signin");

            let b64creds = b64::encode(format!("{}:{}", user, passwd));

            let req = self.client.request(ReqwMethod::POST, url)
                .header("Authorization", format!("Basic {}", b64creds));

            debug!("Request: {:#?}", req);

            let rep = req.send()?;

            match rep.status().as_u16()
            {
                204 => {
                    if let Some(cookie) = rep.headers().get("Set-Cookie")
                    {
                        let session = {
                            if let Ok(s) = cookie.to_str() {
                                s.to_owned()
                            } else {
                                Err(format!("Failed to extract session cookie string: {:#?}", cookie))?
                            }
                        };

                        self.creds = Credentials::Basic {user: user.clone(), passwd: passwd.clone(), cookie: Some(session)};
                    }
                    else {
                        return Err("Missing session cookie after successfull basic auth".into());
                    }
                }

                401 => { Err(InfluxError::AuthUnauthorized(rep.json::<ApiGenericError>()?))?; }
                403 => { Err(InfluxError::AuthAccountDisabled(rep.json::<ApiGenericError>()?))?; }
                _   => { Err(InfluxError::AuthUnknown(rep.json::<ApiGenericError>()?))?; }
            }
        }

        Ok(())
    }

    fn inject_credentials(&self, builder: ReqwRequestBuilder) -> InfluxResult<ReqwRequestBuilder>
    {
        match &self.creds
        {
            Credentials::Basic{user: _, passwd: _, cookie: None} => {
                Err("Missing session cookie from basic auth. This should not have happened!".into())
            }

            Credentials::Basic{user: _, passwd: _, cookie: Some(session)} => {
                Ok(builder.header("Cookie", session))
            }

            Credentials::Token{token} => {
                Ok(builder.header("Authorization", format!("Token {}", token)))
            }
        }
    }
}
