//!
//! Precision of the Measurement being Stored/Loaded
//!
use crate::InfluxError;

use std::fmt;


#[derive(Debug, Clone, Copy)]
pub enum Precision
{
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
}


impl std::str::FromStr for Precision
{
    type Err = InfluxError;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let p = match s
        {
            "ns" => Precision::Nanoseconds,
             "u" => Precision::Microseconds,
            "ms" => Precision::Milliseconds,
             "s" => Precision::Seconds,

            _ => { return Err(format!("Invalid precision: {}", s).into()) }
        };

        Ok(p)
    }
}


impl fmt::Display for Precision
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Precision::Nanoseconds  => "ns".fmt(f),
            Precision::Microseconds =>  "u".fmt(f),
            Precision::Milliseconds => "ms".fmt(f),
            Precision::Seconds      =>  "s".fmt(f),
        }
    }
}
