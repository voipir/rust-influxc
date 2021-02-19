//!
//! Unit of recording that can contain multiple Measurements
//!
use crate::Precision;
use crate::Measurement;


#[derive(Debug)]
pub struct Record
{
    pub org:       String,
    pub bucket:    String,
    pub precision: Precision,

    pub(crate) measurements: Vec<Measurement>,
}


impl Record
{
    pub fn new(org: &str, bucket: &str, precision: Precision) -> Self
    {
        Self {org: org.to_owned(), bucket: bucket.to_owned(), precision, measurements: Vec::new()}
    }

    pub fn measurement<'r>(&'r mut self, name: &str) -> &'r mut Measurement
    {
        self.measurements.push(Measurement::new(name));
        self.measurements.last_mut().unwrap()
    }

    pub fn measurements(&self) -> &[Measurement]
    {
        &self.measurements
    }

    pub fn to_lines(&self) -> Vec<String>
    {
        let mut lines = Vec::new();

        for measurement in self.measurements.iter() {
            lines.push(measurement.to_line(&self.precision));
        }

        lines
    }

    pub fn to_line_buffer(&self) -> String
    {
        self.to_lines()
            .join("\n")
    }
}
