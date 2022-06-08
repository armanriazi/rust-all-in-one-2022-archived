
#![allow(unused)]
fn main() {
// The rest of these are errors, since they have type or const parameters that
// do not constrain.

// T does not constrain since it does not appear at all.
impl<T> Struct { /* ... */ }

// N does not constrain for the same reason.
impl<const N: usize> Struct { /* ... */ }

// Usage of T inside the implementation does not constrain the impl.
impl<T> Struct {
    fn uses_t(t: &T) { /* ... */ }
}

// T is used as an associated type in the bounds for U, but U does not constrain.
impl<T, U> Struct where U: HasAssocType<Ty = T> { /* ... */ }

// T is used in the bounds, but not as an associated type, so it does not constrain.
impl<T, U> GenericTrait<U> for u32 where U: GenericTrait<T> {}
}

/* 
Example of an allowed unconstraining lifetime parameter:
*/


impl<'a> Struct {}
/* 
Example of a disallowed unconstraining lifetime parameter:
*/


impl<'a> HasAssocType for Struct {
    type Ty = &'a Struct;
}