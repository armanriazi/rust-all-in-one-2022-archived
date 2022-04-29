use std::fs::File;
use std::io::ErrorKind;

use std::io::Error;
fn main() -> Result<(), Error>{
    let f = File::open("hello.txt");

   if let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
   
   Ok(())
}
