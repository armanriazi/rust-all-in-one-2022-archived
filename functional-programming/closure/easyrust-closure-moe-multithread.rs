// fn main() {
//     let mut my_string = String::from("Can I go inside the thread?");

//     let handle = std::thread::spawn(move || {
//         println!("{}", my_string); // ⚠️
//     });

//     handle.join();
// }

// fn main() {
//     let mut my_string = String::from("Can I go inside the thread?");

//     let handle = std::thread::spawn(move || {
//         println!("{}", my_string); // now my_string is being used as a reference
//     });

//     std::mem::drop(my_string);  // ⚠️ We try to drop it here. But the thread still needs it.//we have better use {} instead of drop

//     handle.join();
// }
