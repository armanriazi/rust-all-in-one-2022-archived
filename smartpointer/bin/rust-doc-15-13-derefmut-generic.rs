/*Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:
*/

/* 
From &T to &U when T: Deref<Target=U>
From &mut T to &mut U when T: DerefMut<Target=U>
From &mut T to &U when T: Deref<Target=U>
*/
use std::ops::{Deref, DerefMut};

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> 
{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
//implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref. Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
