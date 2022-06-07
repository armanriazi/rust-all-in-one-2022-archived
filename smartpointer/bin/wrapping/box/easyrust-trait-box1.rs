#![allow(dead_code)] // Tell the compiler to be quiet
use std::mem::size_of; // This gives the size of a type

trait JustATrait {} // We will implement this on everything

enum EnumOfNumbers {
    I8(i8),
    AnotherI8(i8),
    OneMoreI8(i8),
}
impl JustATrait for EnumOfNumbers {}

struct StructOfNumbers {
    an_i8: i8,
    another_i8: i8,
    one_more_i8: i8,
}
impl JustATrait for StructOfNumbers {}

enum EnumOfOtherTypes {
    I8(i8),
    AnotherI8(i8),
    Collection(Vec<String>),
}
impl JustATrait for EnumOfOtherTypes {}

struct StructOfOtherTypes {
    an_i8: i8,
    another_i8: i8,
    a_collection: Vec<String>,
}
impl JustATrait for StructOfOtherTypes {}

struct ArrayAndI8 {
    array: [i8; 1000], // This one will be very large
    an_i8: i8,
    in_u8: u8,
}
impl JustATrait for ArrayAndI8 {}

fn returns_just_a_trait() -> Box<dyn JustATrait> {
    let some_enum = EnumOfNumbers::I8(8);
    Box::new(some_enum)
}
fn main() {
    println!(
        "{}, {}, {}, {}, {}",
        size_of::<EnumOfNumbers>(),
        size_of::<StructOfNumbers>(),
        size_of::<EnumOfOtherTypes>(),
        size_of::<StructOfOtherTypes>(),
        size_of::<ArrayAndI8>(),
    );
}
