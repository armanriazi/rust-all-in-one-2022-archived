
/// #The compiler prefers to capture a closed-over variable by immutable borrow, followed by unique immutable borrow (see below), by mutable borrow, and finally by move.
/// # If, instead, the closure were to use self.vec directly, then it would attempt to capture self by mutable reference. 
/// But since self.set is already borrowed to iterate over, the code would not compile.#
#![allow(unused)]
fn main() {
use std::collections::HashSet;

struct SetVec {
    set: HashSet<u32>,
    vec: Vec<u32>
}

impl SetVec {
    fn populate(&mut self) {
        let vec = &mut self.vec;
        self.set.iter().for_each(|&n| {
            vec.push(n);
        })
    }
}
}
