// errorsn.rs
// This is a bigger error exercise than the previous ones!
// You can do it! :)
//
// Edit the `read_and_validate` function so that it compiles and
// passes the tests... so many things could go wrong!
//
// - Reading from stdin could produce an io::Error
// - Parsing the input could produce a num::ParseIntError
// - Validating the input could produce a CreationError (defined below)
//
// How can we lump these errors into one general error? That is, what
// type goes where the question marks are, and how do we return
// that type from the body of read_and_validate?
//
// Execute `rustlings hint errorsn` for hints :)

use std::error;
use std::fmt;
use std::io;//::{self,BufRead};

// PositiveNonzeroInteger is a struct defined below the tests.
fn read_and_validate(
    b: &mut dyn io::BufRead,
) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut line = String::new();
    b.read_line(&mut line)?;//decode. to string
    let num: i64 = line.trim().parse()?;
    let answer = PositiveNonzeroInteger::new(num)?;
    Ok(answer)
    
}

// fn read_numbers_from_file(
//     file: &mut dyn io::BufRead,
// ) -> Result<Vec<i64>, io::Error> {
//     let mut numbers = vec![];
//     for line_result in file.lines() {        
//         let line = line_result?;
//         numbers.push(line.parse()?);
//     }
//     Ok(numbers)
// }

// This is a test helper function that turns a &str into a BufReader.
fn test_with_str(s: &str) -> Result<PositiveNonzeroInteger, Box<dyn error::Error>> {
    let mut b = io::BufReader::new(s.as_bytes());//encode
    read_and_validate(&mut b)
}

fn test_success() {
    let x = test_with_str("42\n");    
    //Ok(PositiveNonzeroInteger(42))
    assert_eq!(PositiveNonzeroInteger(42), x.unwrap());
}


fn test_not_num() {
    let x = test_with_str("eleven billion\n");
    //:Err(ParseIntError { kind: InvalidDigit })    
    //println!("Printed:{:?}",x);
    
    assert!(x.is_err());
}


fn test_non_positive() {
    let x = test_with_str("-40\n");
    //:Err(ParseIntError { kind: InvalidDigit })
    assert!(x.is_err());
}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value == 0 {
            Err(CreationError::Zero)
        } else if value < 0 {
            Err(CreationError::Negative)
        } else {
            Ok(PositiveNonzeroInteger(value as u64))
        }
    }
    
}


fn test_positive_nonzero_integer_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str((self as &dyn error::Error).description())
    }
    
}

impl error::Error for CreationError {
    fn description(&self) -> &str {
        match *self {
            CreationError::Negative => "Negative",
            CreationError::Zero => "Zero",
        }
    }
}

fn main() {//-> Result<(), ParseIntError>
   //let item=func()?;
   test_success();
   test_not_num();
   test_not_num();
   test_positive_nonzero_integer_creation();
   test_non_positive();  
   //ok(())
}