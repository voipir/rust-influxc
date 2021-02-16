//!
//! Measurement to be Stored
//!
use crate::Value;

use crate::ChronoDateTime;

use std::collections::BTreeMap;


#[derive(Debug)]
pub struct Measurement
{
    pub name:      String,
    pub tags:      BTreeMap<String, String>,
    pub fields:    BTreeMap<String, Value>,
    pub timestamp: Option<ChronoDateTime>,
}


impl Measurement
{
    pub fn new(name: &str) -> Self
    {
        Self {
            name: name.to_owned(),

            tags:   BTreeMap::new(),
            fields: BTreeMap::new(),

            timestamp: None,
        }
    }

    pub fn timestamp(mut self, timestamp: ChronoDateTime) -> Self
    {
        self.timestamp = Some(timestamp); self
    }

    pub fn add_tag(mut self, key: &str, value: &str) -> Self
    {
        self.tags.insert(key.to_owned(), value.to_owned());
        self
    }

    pub fn add_field(mut self, key: &str, value: Value) -> Self
    {
        self.fields.insert(key.to_owned(), value);
        self
    }

    pub fn to_line(&self) -> String
    {
        let mut line = self.name.to_owned();

        if ! self.tags.is_empty()
        {
            let tagline = self.tags.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(",");

            line += ",";
            line += &tagline;
        }

        if ! self.fields.is_empty()
        {
            let fieldline = self.fields.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join(",");

            line += " ";
            line += &fieldline;
        }

        // if let Some(ts) = self.timestamp
        // {
        //     line += " ";

        //     match self.precision
        //     {
        //         Precision::Nanoseconds  => { line += &ts.timestamp_nanos().to_string();  }
        //         Precision::Microseconds => { line += &(ts.timestamp_nanos() * 1000).to_string(); }
        //         Precision::Milliseconds => { line += &ts.timestamp_millis().to_string(); }
        //         Precision::Seconds      => { line += &(ts.timestamp() ).to_string(); }
        //         Precision::Minutes      => { line += &(ts.timestamp() /   60).to_string(); }
        //         Precision::Hours        => { line += &(ts.timestamp() / 3600).to_string(); }
        //     }
        // }

        line
    }
}

