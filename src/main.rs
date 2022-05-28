#![feature(lint_reasons)]

#![allow(clippy::some_lint, reason = "False positive rust-lang/rust-clippy#1002020")]

#![allow(unused)]
fn main() {
let mut b = false;
let x = &mut b;

    let mut c = || { *x = true; };
    // The following line is an error:
     let y = &x;
    c();

let z = &x;
}
