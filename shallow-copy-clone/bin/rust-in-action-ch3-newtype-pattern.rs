#[derive(PartialEq)] // <1>
struct Hostname(String); // <2>

fn main() {
    let ordinary_string = String::from("localhost"); 
    let host = Hostname ( ordinary_string.clone() );
    if host.0 == ordinary_string { // <3>
      println!("huh?{}",{ordinary_string});
    };
}