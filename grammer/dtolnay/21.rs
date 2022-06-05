trait Trait {
    fn f(&self);
}

impl<F: FnOnce() -> bool> Trait for F {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for () {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    let x = || { (return) || true; };
    x().f();

    let x = loop { (break) || true; };
    x.f();

    let x = || { return (|| true); };
    x().f();

    let x = loop { break (|| true); };
    x.f();

    let x = || { return || true; };
    x().f();

    let x = loop { break || true; };
    x.f();
}
/*
We want to know whether each possible parenthesization of return || true; and break || true; evaluates to the closure || true or to the unit value ().

let x = || { (return) || true; };

On this line, x is a closure that returns (). It is equivalent to let x = || {}. When we call x().f(), the method f resolves to impl Trait for () which prints 2.

The type of the expression (return) is the primitive never type, usually written as !. It is legal to compute ! || true because ! can fill in for any type, in this case bool. The expression ! || true is a logical-OR with bool on both the left-hand side and right-hand side.

The behavior of ! of filling in for any type is what allows us to write:

fn f() -> bool {
    unimplemented!()
}
in which the type of unimplemented!(), since it panics without evaluating to any value, is also !.

let x = loop { (break) || true; };

Similar to (return), the type of (break) is the never type !. This code breaks out of the loop with the implicit value (), so x is of type (). Calling x.f() will print 2.

let x = || { return (|| true); };

On this line x is a closure that returns a closure that returns true. You could write x()() and that would be true.

The quiz code calls x().f() which resolves to impl<F> Trait for F where F: FnOnce() -> bool. That trait impl prints 1.

let x = loop { break (|| true); };

This is a loop containing a break-with-value expression. The argument of the break becomes the value of the enclosing loop. This code is equivalent to let x = || true.

When we call x.f() it uses the FnOnce impl of Trait which prints 1.

let x = || { return || true; };

Now we arrive at the meat of this quiz question. Is return || true parsed the same as (return) || true or as return (|| true)?

It turns out to be the latter, so x is a closure that returns a closure that returns true. x().f() prints 1.

let x = loop { break || true; };

Similar question here, is this (break) || true or break (|| true)?

The break-with-value language feature was added to Rust more than two years after 1.0, in Rust 1.19. Prior to break-with-value, break || true was perfectly legal Rust code that parsed as (break) || true.

In Rust 1.19 the behavior of this code was unintentionally broken by the language such that now it parses as break (|| true) and the printed value is 1.

If we had noticed this change in meaning during the development of Rust 1.19, we may have adjusted the parsing to preserve the meaning of existing code. Unfortunately doing so would result in a grammar that behaves differently between return and break for no justifiable reason other than an accident of history.

Or it is possible we would have ruled this an edge case of syntax that would never appear in real code, used Crater to validate that hypothesis, and broken the behavior intentionally.

The total output from main is 221111.
*/