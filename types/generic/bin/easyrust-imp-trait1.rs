// fn gives_higher_i32(one: i32, two: i32) {
//     let higher = if one > two { one } else { two };
//     println!("{} is higher.", higher);
// }

// fn main() {
//     gives_higher_i32(8, 10);
// }


/* 
use std::fmt::Display;

fn gives_higher_i32<T: PartialOrd + Display>(one: T, two: T) {
    let higher = if one > two { one } else { two };
    println!("{} is higher.", higher);
}

fn main() {
    gives_higher_i32(8, 10);
}
*/

fn prints_it<T>(input:T) where 
T: Into<String> + std::fmt::Display
{
println!("{}",input);
}

fn main() {
    let name = "Tuon";
    let string_name = String::from("Tuon");
    prints_it(name);
    prints_it(string_name);
}
