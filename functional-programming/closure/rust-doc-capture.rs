/*
The closure then has ownership of x, and main isn’t allowed to use x anymore in the println! statement. Removing println! will fix this example.
*/
// fn main() {
//     let x = vec![1, 2, 3];

//     let equal_to_x = move |z| z == x;

//     println!("can't use x here: {:?}", x);

//     let y = vec![1, 2, 3];

//     assert!(equal_to_x(y));
// }


// fn main() {
//     let x = 4;

//     fn equal_to_x(z: i32) -> bool {
//         z == x
//     }

//     let y = 4;

//     assert!(equal_to_x(y));
// }


// fn main() {
//     let x = 4;

//     let equal_to_x = |z| z == x;
//        println!("can use x here: {:?}", x);
//     let y = 4;

//     assert!(equal_to_x(y));
// }
