trait Trait {
    fn f(&self);
}

impl<'a> dyn Trait + 'a {
    fn f(&self) {
        print!("1");
    }
}

impl Trait for bool {
    fn f(&self) {
        print!("2");
    }
}

fn main() {
    Trait::f(&true);
    Trait::f(&true as &dyn Trait);
    <_ as Trait>::f(&true);
    <_ as Trait>::f(&true as &dyn Trait);
    <bool as Trait>::f(&true);
}
/*
22222
This question contains a trait method Trait::f as well as an inherent method f on the trait object type dyn Trait.

As far as I know, given that these names shadow each other, the inherent method is literally uncallable. There is currently no syntax in Rust for calling the inherent f on dyn Trait.

If the trait method were named something different and only the inherent method were called f, then the first two lines of main would successfully call the inherent method. However, as written with shadowed names, they disambiguate to the trait method.

One additional syntax to try would be:

<dyn Trait>::f(&true);
<dyn Trait>::f(&true as &dyn Trait);
If the trait method were named something different, both of these would call the inherent method. If the inherent method were named something different, both of these would call the trait method. But if the trait method and the inherent method are both f then the compiler reports an ambiguity.

error[E0034]: multiple applicable items in scope
  --> questions/010.rs:18:5
   |
18 |     <dyn Trait>::f(&true);
   |     ^^^^^^^^^^^^^^ multiple `f` found
   |
note: candidate #1 is defined in an impl for the type `dyn Trait`
  --> questions/010.rs:6:5
   |
6  |     fn f(&self) {
   |     ^^^^^^^^^^^
note: candidate #2 is defined in the trait `Trait`
  --> questions/010.rs:2:5
   |
2  |     fn f(&self);
   |     ^^^^^^^^^^^^
   = help: to disambiguate the method call, write `Trait::f(...)` instead
Maybe some day it will be possible to disambiguate a call to an inherent method on a trait object shadowed by a trait method. For now, the quiz code prints 22222.
*/