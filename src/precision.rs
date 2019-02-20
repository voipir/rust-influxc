//!
//! Precision of the Measurement being Stored/Loaded
//!
use std::fmt;


#[derive(Debug, Serialize, Deserialize)]
pub enum Precision
{
    #[serde(rename="ns")] Nanoseconds,
    #[serde(rename="us")] Microseconds,
    #[serde(rename="ms")] Milliseconds,
    #[serde(rename="s")]  Seconds,
    #[serde(rename="m")]  Minutes,
    #[serde(rename="h")]  Hours,
}


impl fmt::Display for Precision
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        match self
        {
            Precision::Nanoseconds  => "ns".fmt(f),
            Precision::Microseconds => "us".fmt(f),
            Precision::Milliseconds => "ms".fmt(f),
            Precision::Seconds      =>  "s".fmt(f),
            Precision::Minutes      =>  "m".fmt(f),
            Precision::Hours        =>  "h".fmt(f),
        }
    }
}
