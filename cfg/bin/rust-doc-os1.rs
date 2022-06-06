
#![allow(unused)]
#![crate_name = "rust_all_in_one"]
#[cfg(panic = "unwind")]

fn main()->impl std::process::Termination {
let machine_kind = if cfg!(unix) {
  println!("I'm running on a {} machine!", "unix");
  std::process::ExitCode::SUCCESS
  
} else if cfg!(windows) {
    println!("I'm running on a {} machine!", "windows");
    std::process::ExitCode::SUCCESS
} else {  
  println!("I'm running on a {} machine!", "unknown");
  std::process::ExitCode::FAILURE
};


}
