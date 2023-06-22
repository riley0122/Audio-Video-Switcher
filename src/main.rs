use std::fs::File;
use std::io::{BufReader};
use mp4::{Result};

fn main() -> Result<()> {
    let FILE_LOCATION = "./Sample.mp4";

    let thefile = File::open(FILE_LOCATION).unwrap();
    let size = thefile.metadata()?.len();
    let reader = BufReader::new(thefile);
    let mp4 = mp4::Mp4Reader::read_header(reader, size)?;

    Ok(())
}
