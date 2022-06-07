use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {} // Now it is an error type with Debug. Time for Display:

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!") // All it does is write this message
    }
}


#[derive(Debug)] // Do the same thing with ErrorTwo
struct ErrorTwo;

impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

// Make a function that just returns a String or an error
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> { // With Box<dyn Error> you can return anything that has the Error trait

    match input {
        0 => Err(Box::new(ErrorOne)), // Don't forget to put it in a box
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()), // This is the success type
    }

}

fn main() {

    let vec_of_u8s = vec![0_u8, 1, 80]; // Three numbers to try out

    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}
