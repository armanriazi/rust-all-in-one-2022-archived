use std::fmt::{self, Display};

struct S;

impl Display for S {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("1")
    }
}

impl Drop for S {
    fn drop(&mut self) {
        print!("2");
    }
}

fn f() -> S {
    S
}

fn main() {
    let S = f();
    print!("{}", S);
}
/*
212
This program prints 212.

No value of type S gets dropped within the body of function f. The function f conjures an S and returns ownership of it to the caller of f; the caller determines when to drop the S of which it received ownership.

On the first line of main, we call f() and perform an infallible match that binds no new variables. As no variables are declared on this line, there is no variable that could be the owner of the S returned by f() so that S is dropped at that point, printing 2. The S in let S = f() is a unit struct pattern (not a variable name) that matches a value of type S via destructuring but does not bind the value to any variable.

The second line of main conjures a new S, prints it, and drops it at the semicolon.
*/