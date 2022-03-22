
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn main() {

    let st="arman";
    let result = longest_with_an_announcement("2","5",&st);

    println!("The longest string is {}", result);
}