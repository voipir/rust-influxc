//!
//! Unit of recording that can contain multiple Measurements
//!
use crate::Precision;
use crate::Measurement;


#[derive(Debug)]
pub struct Record
{
    bucket:    String,
    precision: Precision,

    measurements: Vec<Measurement>,
}


impl Record
{
    fn new(bucket: &str, precision: Precision) -> Self
    {
        Self {bucket: bucket.to_owned(), precision, measurements: Vec::new()}
    }

    fn measurement<'r>(&'r mut self, name: &str) -> &'r mut Measurement
    {
        self.measurements.push(Measurement::new(name));
        self.measurements.last_mut().unwrap()
    }
}
