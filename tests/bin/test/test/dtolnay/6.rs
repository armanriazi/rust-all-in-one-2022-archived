use std::mem;

fn main() {
    let a;
    let a = a = true;
    print!("{}", mem::size_of_val(&a));
}
/*
0
There are two variables named a, one shadowing the other. The program is equivalent to:

let a;
let b = a = true;
print!("{}", mem::size_of_val(&b));
Further, the value being assigned to b is the expression a = true.

In Rust, assignment expressions always have the value (). Simplified some more, the quiz code is equivalent to:

let a = true;
let b = ();
print!("{}", mem::size_of_val(&b));
Refer to the documentation of size_of_val for a specification of its behavior, but in this case it is being instantiated with T = () and we end up printing the value of size_of::<()>().

() is one example of a zero-sized type or ZST and is represented by zero bytes of data at runtime, so the program prints 0.
*/