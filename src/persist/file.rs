//!
//! Persistant side storage for failed inserts "after the fact" as important distinction to a "write ahead log"
//! thus aiming to be more flash friendly on embedded devices where you want to keep writes to a minimum. This
//! will cause data loss if after failed insert also the writing to flash fails!. For that prefer a WAL approach.
//!
//! WAL approach: TODO
//!
use crate::Client;
use crate::ClientTrait;
use crate::Measurement;

use crate::InfluxError;
use crate::InfluxResult;

use crate::json;

use std::fs::File;
use std::fs::OpenOptions;

use std::io::Seek;
use std::io::SeekFrom;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::io::BufWriter;


pub struct FileBackloggedClient
{
    client: Client,

    path:   String,
    handle: File,
    count:  usize,
}


impl FileBackloggedClient
{
    pub fn new(client: Client, path: String) -> InfluxResult<Self>
    {
        let handle = OpenOptions::new()
            .read(true)
            .append(true)
            .create(true)
            .open(&path)?;

        let bfrd = BufReader::new(&handle);

        let count = bfrd.lines()
            .count();

        info!("Influx backlog has {} backlogged entries waiting to be written to database", count);

        Ok(Self {client, path, handle, count})
    }

    pub fn commit_measurements(&mut self) -> InfluxResult<()>
    {
        info!("Working off infux backlog of {} entries", self.count);

        let points = self.read_measurements()?;

        if let Err(e) = self.client.write_many(&points) {
            Err(InfluxError::Error(format!("Unable to commit backlogged measurements: {}", e)))
        } else {
            if let Err(e) = self.truncate_measurements()
            {
                let msg = format!("Failed to eliminate/truncate measurements from file: {}", e);
                error!("{}", msg);
                panic!("{}", msg);
            } else {
                Ok(())
            }
        }
    }
}


/// private interface
impl FileBackloggedClient
{
    fn read_measurements(&mut self) -> InfluxResult<Vec<Measurement>>
    {
        self.handle.seek(SeekFrom::Start(0))?;  // go to begining of file

        let mut points = Vec::new();
        let     bfrd   = BufReader::new(&self.handle);

        for (num, line) in bfrd.lines().enumerate()
        {
            let ln = line?;

            match json::from_str(&ln)
            {
                Ok(point) => {
                    points.push(point)
                }

                Err(e) => {
                    error!("Failed to read line {}", num);
                    return Err(e.into());
                }
            }
        }

        Ok(points)
    }

    fn write_measurements(&mut self, points: &[Measurement]) -> InfluxResult<()>
    {
        self.handle.seek(SeekFrom::End(0))?;  // go to end of file

        let mut bfwr = BufWriter::new(&self.handle);

        for point in points
        {
            let line = json::to_string(point)?;

            bfwr.write(line.as_bytes())?;
            bfwr.write(b"\n")?;

            self.count += 1;
        }

        bfwr.flush()?;

        Ok(())
    }

    fn truncate_measurements(&mut self) -> InfluxResult<()>
    {
        self.handle = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(&self.path)?;

        self.count = 0;

        Ok(())
    }
}


impl ClientTrait for FileBackloggedClient
{
    fn write_one(&mut self, point: Measurement) -> InfluxResult<()>
    {
        self.write_many(&[point])?;

        Ok(())
    }

    /// FIXME currently if connection comes on and off during backlog processing, it will not truncate. This means that
    /// all processed backlog measurements will be processed again once connection comes back up. InfluxDB will
    /// deduplicate this automatically, but it can explode the file due to never being truncated on flaky connections.
    /// Also the longer the backlog becomes, less the probability of it being truncated due to the increased lenght/time
    /// required to play back onto the DB.
    fn write_many(&mut self, points: &[Measurement]) -> InfluxResult<()>
    {
        if let Err(e) = self.client.write_many(points)
        {
            error!("Error while inserting to influxdb, proceeding to append to backlog: {}", e);

            if let Err(e) = self.write_measurements(points)
            {
                let msg = format!("Failed to write measurements to file: {}", e);
                error!("{}", msg); panic!("{}", msg);
            }
        }
        else
        {
            if self.count > 0 {
                self.commit_measurements().ok();
            }
        }

        Ok(())
    }
}
