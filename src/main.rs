use std::fs::File;
use std::io::{BufReader};
use mp4::{Result};

fn main() -> Result<()> {
    let FILE_LOCATION = "./Sample.mp4";

    let thefile = File::open(FILE_LOCATION);

    Ok(())
}
