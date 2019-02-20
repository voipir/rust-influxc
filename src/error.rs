//!
//! Error Handling
//!
#![allow(deprecated)]

use crate::JsonError;

use crate::ReqwError;
use crate::ReqwUrlError;

use std::io::Error as StdIoError;


error_chain!
{
    types
    {
        InfluxError,
        InfluxErrorKind,
        InfluxResultExt,
        InfluxResult;
    }

    links
    {

    }

    foreign_links
    {
        Io(StdIoError);
        Json(JsonError);
        Reqwest(ReqwError);
        ReqwestUrl(ReqwUrlError);
    }
}
