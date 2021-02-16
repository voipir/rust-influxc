//!
//! Persistant side storage for failed inserts "after the fact" as important distinction to a "write ahead log"
//! thus aiming to be more flash friendly on embedded devices where you want to keep writes to a minimum. This
//! will cause data loss if after failed insert also the writing to flash fails!. For that prefer a WAL approach.
//!
//! WAL approach: TODO
//!
use super::Backlog;

use crate::Record;

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


#[derive(Debug)]
pub struct FileBacklog
{
    path:   String,
    handle: File,
    count:  usize,
}


impl FileBacklog
{
    pub fn new(path: String) -> InfluxResult<Self>
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

        Ok(Self {path, handle, count})
    }
}


impl Backlog for FileBacklog
{
    fn read_pending(&mut self) -> InfluxResult<Vec<Record>>
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

    fn write_pending(&mut self, points: &[Record]) -> InfluxResult<()>
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

    fn truncate_pending(&mut self) -> InfluxResult<()>
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
