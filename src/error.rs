//!
//! Error Handling
//!
use crate::JsonError;

use crate::ReqwError;

use crate::Deserialize;

pub type InfluxResult<T> = Result<T, InfluxError>;


pub trait InfluxErrorAnnotate<T>
{
    fn annotate<M: ToString>(self, msg: M) -> InfluxResult<T>;
}


#[derive(Debug)]
pub enum InfluxError
{
    // generic
    Error(String),
    Annotated(String, Box<InfluxError>),

    // stdlib
    Io(std::io::Error),
    ParseBool(std::str::ParseBoolError),

    // 3d party
    Json(JsonError),
    Reqwest(ReqwError),

    // /api/v2/signin
    AuthUnauthorized(ApiGenericError),
    AuthAccountDisabled(ApiGenericError),
    AuthUnknown(ApiGenericError),

    // /api/v2/write
    WriteMalformed(ApiMalformationError),

    WriteUnauthorized(ApiGenericError),
    WriteUnauthenticated(ApiGenericError),
    WriteOversized(ApiOversizeError),

    WriteOverquota(ApiDelayError),
    WriteUnready(ApiDelayError),
    WriteUnknown(ApiGenericError),

    // /api/v2/query - TODO
}


#[derive(Debug, Deserialize)]
pub struct ApiGenericError
{
    code:    String,
    message: String,
}


#[derive(Debug, Deserialize)]
pub struct ApiDelayError
{
    delay: i64,
}


#[derive(Debug, Deserialize)]
pub struct ApiMalformationError
{
    code:    String,
    err:     String,
    line:    Option<i32>,
    message: String,
    op:      String,
}


#[derive(Debug, Deserialize)]
pub struct ApiOversizeError
{
    code: String,

    #[serde(rename="maxLength")]
    maxlen:  i32,

    message: String,
}


impl<T, E> InfluxErrorAnnotate<T> for Result<T, E>
    where E: Into<InfluxError> + std::error::Error
{
    fn annotate<M: ToString>(self, msg: M) -> InfluxResult<T>
    {
        self.map_err(|e| {
            InfluxError::Annotated(msg.to_string(), Box::new(e.into()))
        })
    }
}


impl From<&str>   for InfluxError { fn from(err: &str)   -> InfluxError { InfluxError::Error(err.to_owned()) }}
impl From<String> for InfluxError { fn from(err: String) -> InfluxError { InfluxError::Error(err) }}

impl From<std::io::Error>           for InfluxError { fn from(err: std::io::Error)           -> InfluxError { InfluxError::Io(err) }}
impl From<std::str::ParseBoolError> for InfluxError { fn from(err: std::str::ParseBoolError) -> InfluxError { InfluxError::ParseBool(err) }}

impl From<JsonError> for InfluxError { fn from(err: JsonError) -> InfluxError { InfluxError::Json(err) }}
impl From<ReqwError> for InfluxError { fn from(err: ReqwError) -> InfluxError { InfluxError::Reqwest(err) }}


impl std::fmt::Display for InfluxError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match *self
        {
            Self::Error(ref err)        => { write!(f, "{}", err) }
            Self::Annotated(ref msg, _) => { write!(f, "{}", msg) },

            Self::Io(ref err)        => { write!(f, "Io Error: {}",      err) }
            Self::ParseBool(ref err) => { write!(f, "Parse Bool Error: {}",      err) }

            Self::Json(ref err)       => { write!(f, "Json Error: {}",    err) }
            Self::Reqwest(ref err)    => { write!(f, "Reqwest Error: {}", err) }

            Self::AuthUnauthorized(ref inner)     => { write!(f, "AuthUnauthorized({})",     inner) }
            Self::AuthAccountDisabled(ref inner)  => { write!(f, "AuthAccountDisabled({})",  inner) }
            Self::AuthUnknown(ref inner)          => { write!(f, "AuthUnknown({})",          inner) }
            Self::WriteMalformed(ref inner)       => { write!(f, "WriteMalformed({})",       inner) }
            Self::WriteUnauthorized(ref inner)    => { write!(f, "WriteUnauthorized({})",    inner) }
            Self::WriteUnauthenticated(ref inner) => { write!(f, "WriteUnauthenticated({})", inner) }
            Self::WriteOversized(ref inner)       => { write!(f, "WriteOversized({})",       inner) }
            Self::WriteOverquota(ref inner)       => { write!(f, "WriteOverquota({})",       inner) }
            Self::WriteUnready(ref inner)         => { write!(f, "WriteUnready({})",         inner) }
            Self::WriteUnknown(ref inner)         => { write!(f, "WriteUnknown({})",         inner) }
        }
    }
}


impl std::fmt::Display for ApiGenericError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "code={}, message={}", self.code, self.message)
    }
}


impl std::fmt::Display for ApiDelayError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "delay={}s", self.delay)
    }
}


impl std::fmt::Display for ApiMalformationError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "code={}, err={}, line={}, message={}, op={}",
            self.code,
            self.err,
            self.line.map(|v| v.to_string()).unwrap_or_else(|| "n/a".to_owned()),
            self.message,
            self.op
        )
    }
}


impl std::fmt::Display for ApiOversizeError
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "code={}, maxlen={}, message={}", self.code, self.maxlen, self.message)
    }
}


impl std::error::Error for InfluxError
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)>
    {
        match *self
        {
            InfluxError::Error(_)              => { None }
            InfluxError::Annotated(_, ref err) => { Some(err) }

            InfluxError::Io(ref err)        => { Some(err) }
            InfluxError::ParseBool(ref err) => { Some(err) }

            InfluxError::Json(ref err)    => { Some(err) }
            InfluxError::Reqwest(ref err) => { Some(err) }

            InfluxError::AuthUnauthorized(_)     => { None }
            InfluxError::AuthAccountDisabled(_)  => { None }
            InfluxError::AuthUnknown(_)          => { None }
            InfluxError::WriteMalformed(_)       => { None }
            InfluxError::WriteUnauthorized(_)    => { None }
            InfluxError::WriteUnauthenticated(_) => { None }
            InfluxError::WriteOversized(_)       => { None }
            InfluxError::WriteOverquota(_)       => { None }
            InfluxError::WriteUnready(_)         => { None }
            InfluxError::WriteUnknown(_)         => { None }
        }
    }
}
