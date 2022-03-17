use std::error::Error;
use std::fs::File;
use std::io::Error;
fn main() -> Result<(), Error> {
    let f = File::open("hello.txt")?;

    Ok(())
}
