trait Or {
    fn f(self);
}

struct T;

impl Or for &T {
    fn f(self) {
        print!("1");
    }
}

impl Or for &&&&T {
    fn f(self) {
        print!("2");
    }
}

fn main() {
    let t = T;
    let wt = &T;
    let wwt = &&T;
    let wwwt = &&&T;
    let wwwwt = &&&&T;
    let wwwwwt = &&&&&T;
    t.f();
    wt.f();
    wwt.f();
    wwwt.f();
    wwwwt.f();
    wwwwwt.f();
}
/*
111222

The Reference describes Rust's method lookup order. The relevant paragraph is:

Obtain [the candidate receiver type] by repeatedly dereferencing the receiver expression's type, adding each type encountered to the list, then finally attempting an unsized coercion at the end, and adding the result type if that is successful. Then, for each candidate T, add &T and &mut T to the list immediately after T.

Applying these rules to the given examples, we have:

t.f(): We try to find a function f defined on the type T, but there is none. Next, we search the type &T, and find the first implemenation of the Or trait, and we are done. Upon invocation, the resolved call prints 1.
wt.f(): We search for a function f defined on &T, which immediately succeeds. Upon invocation, the function prints 1.
wwt.f(): The search order is &&T -> &&&T -> &mut &&T -> &T, and we're done. Upon invocation, the function prints 1.
wwwt.f(): &&&T -> &&&&T. This prints 2.
wwwwt.f(): &&&&T. This prints 2.
wwwwwt.f(): &&&&&T -> &&&&&&T -> &mut &&&&&T -> &&&&T. This prints 2.
*/