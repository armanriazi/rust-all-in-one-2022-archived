fn f<'a>() {}
fn g<'a: 'a>() {}

fn main() {
    let pf = f::<'static> as fn();
    let pg = g::<'static> as fn();
    print!("{}", pf == pg);
}


/*
Er

Function pointer comparison is generally a Bad Idea. It is easily possible to get nonsensical behavior in optimized builds. For a jaw-dropping example of such behavior, check out rust-lang/rust#54685 in which x == y is both true and not true at the same time.

That said, the quiz code in this question fails to compile. Here is the compiler output:

error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
 --> questions/011.rs:5:18
  |
5 |     let pf = f::<'static> as fn();
  |                  ^^^^^^^
  |
note: the late bound lifetime parameter is introduced here
 --> questions/011.rs:1:18
  |
1 | fn f<'a>() {}
  |      ^^
Generic parameters can be either early bound or late bound. Currently (and for the foreseeable future) type parameters are always early bound, but lifetime parameters can be either early or late bound.

Early bound parameters are determined by the compiler during monomorphization. Since type parameters are always early bound, you cannot have a value whose type has an unresolved type parameter. For example:

fn m<T>() {}

fn main() {
    let m1 = m::<u8>; // ok
    let m2 = m; // error: cannot infer type for `T`
}
However, this is often allowed for lifetime parameters:

fn m<'a>(_: &'a ()) {}

fn main() {
    let m1 = m; // ok even though 'a isn't provided
}
Since the actual choice of lifetime 'a depends on how it is called, we are allowed to omit the lifetime parameter and it will be determined at the call site. The lifetime can even be different for each time it gets called.

For this reason, we cannot specify the lifetime on this function until it is called:

// error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
let m2 = m::<'static>;
We may not even ask the borrow checker to infer it too soon:

// error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
let m3 = m::<'_>;
The idea of late bound parameters overlaps considerably with a feature of Rust called "higher ranked trait bounds" (HRTB). This is a mechanism for expressing that bounds on a trait's parameters are late bound. Currently this is limited to lifetime parameters, but the same idea exists in other languages (such as Haskell) for type parameters, which is where the term "higher ranked" comes from.

The syntax to express a HRTB for lifetimes uses the for keyword. To express the type of m1 above, we could have written:

let m1: impl for<'r> Fn(&'r ()) = m;
You can think of this as meaning: "There is a lifetime but we don't need to know what it is just yet".

Late bound lifetimes are always unbounded; there is no syntax for expressing a late bound lifetime that must outlive some other lifetime.

error: lifetime bounds cannot be used in this context
 --> src/main.rs:5:20
  |
5 |     let _: for<'b: 'a> fn(&'b ());
  |                    ^^
Lifetimes on data types are always early bound except when the developer has explicitly used the HRTB for syntax. On functions, lifetimes are late bound by default but can be early bound if:

The lifetime is declared outside the function signature, e.g. in an associated method of a struct it could be from the struct itself; or

The lifetime parameter is bounded below by some other lifetime that it must outlive. As we've seen, this constraint is not expressible in the HRTB that would be involved in late binding the lifetime.

By these rules, the signature fn f<'a>() has a late bound lifetime parameter while the signature fn g<'a: 'a>() has an early bound lifetime parameter — even though the constraint here is ineffectual.

Ordinarily these distinctions are compiler-internal terminology that Rust programmers are not intended to know about or think about in everyday code. There are only a few edge cases where this aspect of the type system becomes observable in the surface language, such as in the original quiz code.
*/