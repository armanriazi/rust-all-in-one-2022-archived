static PI: f64 = 3.1415;

fn main() {
    // static variables can also be scoped to a function
    static mut SECRET: &'static str = "swordfish";

    // string literals have a 'static lifetime
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // You can break some rules, but you must be explicit
    unsafe {
        // we can set SECRET to a string literal because it is also `static
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}
/*
Standard Output
Hello World! 3.1415
abracadabra
*/
