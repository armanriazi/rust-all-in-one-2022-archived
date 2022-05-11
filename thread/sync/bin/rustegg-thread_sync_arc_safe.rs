// arc1.rs
// Make this code compile by filling in a value for `shared_numbers` where the
// TODO comment is and create an initial binding for `child_numbers`
// somewhere. Try not to create any copies of the `numbers` Vec!
// Execute `rustlings hint arc1` for hints :)

use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone();
        joinhandles.push(thread::spawn(move || {
            let mut i = offset;
            let mut sum = 0;
            while i < child_numbers.len() {
                sum += child_numbers[i];
                i += 5;
            }
            println!("Sum of offset {} is {}", offset, sum);
            //if we need to access sum on main thread or external current thread so we need to Arc::New(Mutext::new(0)) that show in the other examples for using Arc with Mutex
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
/*
Sum of offset 0 is 950
Sum of offset 6 is 969
Sum of offset 7 is 988
Sum of offset 5 is 950
Sum of offset 1 is 970
Sum of offset 2 is 990
Sum of offset 4 is 1030
Sum of offset 3 is 1010
*/