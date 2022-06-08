
#![allow(unused)]
fn main() {
// Self: Sized traits are not object-safe.
trait TraitWithSize where Self: Sized {}

struct S;
impl TraitWithSize for S {}
let obj: Box<dyn TraitWithSize> = Box::new(S); // ERROR
}
