//!
//! Unit of recording that can contain multiple Measurements
//!
use crate::Precision;
use crate::Measurement;

use crate::InfluxResult;

use crate::FlateLevel;
use crate::FlateGzipBuilder;

use crate::ReqwRequestBuilder;

use std::io::Write;


#[derive(Debug)]
pub struct Record
{
    pub(crate) org:          String,
    pub(crate) bucket:       String,
    pub(crate) precision:    Precision,
    pub(crate) measurements: Vec<Measurement>,
}


impl Record
{
    pub fn new(org: &str, bucket: &str) -> Self
    {
        Self {
            org:          org.to_owned(),
            bucket:       bucket.to_owned(),
            precision:    Precision::default(),
            measurements: Vec::new()
        }
    }

    pub fn precision(mut self, precision: Precision) -> Self
    {
        self.precision = precision; self
    }

    pub fn measurement<'r>(&'r mut self, name: &str) -> &'r mut Measurement
    {
        self.measurements.push(Measurement::new(name));
        self.measurements.last_mut().unwrap()
    }
}


/// Internal API - TODO hide from docs
impl Record
{
    pub(crate) fn to_lines(&self) -> Vec<String>
    {
        let mut lines = Vec::new();

        for measurement in self.measurements.iter() {
            lines.push(measurement.to_line(&self.precision));
        }

        lines
    }

    pub(crate) fn to_line_buffer(&self) -> String
    {
        self.to_lines()
            .join("\n")
    }

    pub(crate) fn to_write_request(&self, mut builder: ReqwRequestBuilder) -> InfluxResult<ReqwRequestBuilder>
    {
        // buffer compression
        let mut gzipenc = FlateGzipBuilder::new()
            .write(Vec::new(), FlateLevel::default());

        gzipenc.write_all(self.to_line_buffer().as_bytes())?;

        let buffer = gzipenc.finish()?;

        // headers and query path
        builder = builder.header("Content-Encoding", "gzip");

        builder = builder.query(&[
            ("org",       &self.org),
            ("bucket",    &self.bucket),
            ("precision", &self.precision.to_string()),
        ]);

        // buffer body
        Ok(builder.body(buffer))
    }
}


impl std::fmt::Display for Record
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        let lines = self.measurements.iter()
            .map(|m| {
                let tags = m.tags.iter()
                    .map(|(k, v)| format!("{}:{}", k, v))
                    .collect::<Vec<String>>()
                    .join(" ");

                let fields = m.fields.iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<String>>()
                    .join(" ");

                format!("\tmeasurement={} {} {} {}", m.name, tags, fields, m.timestamp)
            })
            .collect::<Vec<String>>()
            .join("\n");


        write!(f, "Record(org={}, bucket={}, precision={})\n{}", self.org, self.bucket, self.precision, lines)
    }
}
