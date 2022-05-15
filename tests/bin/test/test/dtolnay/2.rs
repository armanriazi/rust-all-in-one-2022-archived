struct S(i32);

impl std::ops::BitAnd<S> for () {
    type Output = ();

    fn bitand(self, rhs: S) {
        print!("{}", rhs.0);
    }
}

fn main() {
    let f = || ( () & S(1) );
    let g = || { () & S(2) };
    let h = || ( {} & S(3) );
    let i = || { {} & S(4) };
    f();
    g();
    h();
    i();
}
/*
123
The closures f, g, and h are all of type impl Fn(). The closure bodies are parsed as an invocation of the user-defined bitwise-AND operator defined above by the BitAnd trait impl. When the closures are invoked, the bitwise-AND implementation prints the content of the S from the right-hand side and evaluates to ().

The closure i is different. Formatting the code with rustfmt makes it clearer how i is parsed.

let i = || {
    {}
    &S(4)
};
The closure body consists of an empty block-statement {} followed by a reference to S(4), not a bitwise-AND. The type of i is impl Fn() -> &'static S.

The parsing of this case is governed by this code in libsyntax.
*/
