//! Simulating files one step at a time.

#![allow(dead_code)] // <1> Silence warnings related to FileState::Open not being used

use std::fmt;  // <2> Bring the `std::fmt` crate into local scope, allowing us to make use of `fmt::Result`
use std::fmt::{Display};  // <3> Bring `Display` into local scope, avoiding the need for us to prefix it as `fmt::Display` in our code


#[derive(Debug,PartialEq)]
pub enum FileState {
   Open,
   Closed,
 }
 
 
 
 /// Represents a "file",
 /// which probably lives on a file system.
 #[derive(Debug,PartialEq)]
 pub struct File {
   pub name: String,
   data: Vec<u8>,
   pub state: FileState
 }
 
 impl Display for FileState {
  fn fmt(&self, f:
         &mut fmt::Formatter,
  ) -> fmt::Result {
    match *self {
      FileState::Open => write!(f, "OPEN"),
      FileState::Closed => write!(f, "CLOSED"),
    }
  }
}
 
impl Display for File {
   fn fmt(&self, f:
          &mut fmt::Formatter,
   ) -> fmt::Result {
      write!(f, "<{} ({})>",
             self.name, self.state)
   }
}
trait Read {          
  fn read(self: &Self, save_to: &mut Vec<u8>) -> Result<usize, String>;
}
#[allow(unused_variables)]
impl Read for File {
  fn read(
    self: &File,
    save_to: &mut Vec<u8>,
  ) -> Result<usize, String> {
    if self.state != FileState::Open {
      return Err(String::from("File must be open for reading"));
    }
    let mut tmp = self.data.clone();
    let read_length = tmp.len();
    save_to.reserve(read_length);
    save_to.append(&mut tmp);
    Ok(read_length)
  }
}

impl File {
     /// Creates a new, empty `File`.
   ///
   /// # Examples
   ///
   /// ```
   /// let f = File::new("f1.txt");
   /// ```
     /// New files are assumed to be empty, but a name is required.
   pub fn new(name: &str) -> File {
     File {
       name: String::from(name),
       data: Vec::new(),
       state: FileState::Closed,
     }
   }
   /// Returns the file's length in bytes.
   pub fn len(&self) -> usize {
    self.data.len()
  }

  /// Returns the file's name.
  pub fn name(&self) -> String {
    self.name.clone()
  }
 }


 
 fn open(mut f: File) -> Result<File, String> {
   f.state = FileState::Open;
   Ok(f)
 }
 
 fn close(mut f: File) -> Result<File, String> {
   f.state = FileState::Closed;
   Ok(f)
 }
 
 fn main() {
  let mut f5 = File::new("5.txt"); 

  let mut buffer: Vec<u8> = vec![];

  if f5.read(&mut buffer).is_err() {
    println!("Error checking is working");
  }

  f5 = open(f5).unwrap();                        // <9>
  let f5_length = f5.read(&mut buffer).unwrap(); // <9>
  f5 = close(f5).unwrap();                       // <9>
  
  let text = String::from_utf8_lossy(&buffer);
 

  let f5_name = f5.name();
  let f5_len = f5.len();

  println!("{:?}", f5);
  println!("{} is {} bytes long", f5_name, f5_len);


   println!("{:?}", f5);
   println!("{}", f5);
   println!("{} is {} bytes long", &f5.name, f5_length);
   println!("{}", text);
 }