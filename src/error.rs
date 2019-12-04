//!
//! Error Handling
//!
use crate::JsonError;

use crate::ReqwError;
use crate::ReqwUrlError;

use std::io;
use std::fmt;
use std::error;


pub type InfluxResult<T> = Result<T, InfluxError>;


#[derive(Debug)]
pub enum InfluxError
{
    Error(String),

    Io(io::Error),
    Json(JsonError),
    Reqwest(ReqwError),
    ReqwestUrl(ReqwUrlError),
}


impl From<&str>         for InfluxError { fn from(err: &str)         -> InfluxError { InfluxError::Error(err.to_owned()) }}
impl From<String>       for InfluxError { fn from(err: String)       -> InfluxError { InfluxError::Error(err)            }}
impl From<io::Error>    for InfluxError { fn from(err: io::Error)    -> InfluxError { InfluxError::Io(err)               }}
impl From<JsonError>    for InfluxError { fn from(err: JsonError)    -> InfluxError { InfluxError::Json(err)             }}
impl From<ReqwError>    for InfluxError { fn from(err: ReqwError)    -> InfluxError { InfluxError::Reqwest(err)          }}
impl From<ReqwUrlError> for InfluxError { fn from(err: ReqwUrlError) -> InfluxError { InfluxError::ReqwestUrl(err)       }}


impl fmt::Display for InfluxError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match *self
        {
            InfluxError::Error(ref err)      => { write!(f, "Error: {}", err) }
            InfluxError::Io(ref err)         => { write!(f, "Io Error: {}", err) }
            InfluxError::Json(ref err)       => { write!(f, "Json Error: {}", err) }
            InfluxError::Reqwest(ref err)    => { write!(f, "Reqwest Error: {}", err) }
            InfluxError::ReqwestUrl(ref err) => { write!(f, "Reqwest Url Error: {}", err) }
        }
    }
}


impl error::Error for InfluxError
{
    fn description(&self) -> &str
    {
        match *self
        {
            InfluxError::Error(ref err)      => { &err }
            InfluxError::Io(ref err)         => { err.description() }
            InfluxError::Json(ref err)       => { err.description() }
            InfluxError::Reqwest(ref err)    => { err.description() }
            InfluxError::ReqwestUrl(ref err) => { err.description() }
        }
    }

    fn source(&self) -> Option<&(dyn error::Error + 'static)>
    {
        match *self
        {
            InfluxError::Error(_)            => { None }
            InfluxError::Io(ref err)         => { Some(err) }
            InfluxError::Json(ref err)       => { Some(err) }
            InfluxError::Reqwest(ref err)    => { Some(err) }
            InfluxError::ReqwestUrl(ref err) => { Some(err) }
        }
    }
}
