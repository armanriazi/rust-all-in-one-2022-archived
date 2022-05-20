fn main() {
    let mut x = 4;
    --x;
    print!("{}{}", --x, --x);
}
//44
/*
Unlike C or Java, there is no unary increment or decrement operator in Rust. The Rust language design FAQ (no longer available online) used to touch on the reason:

Why doesn't Rust have increment and decrement operators?
Preincrement and postincrement (and the decrement equivalents), while convenient, are also fairly complex. They require knowledge of evaluation order, and often lead to subtle bugs and undefined behavior in C and C++. x = x + 1 or x += 1 is only slightly longer, but unambiguous.

In the absense of a decrement operator, --x is parsed as -(-x). In the case of x = 4 this would be -(-4) which is 4. The program is equivalent to:

fn main() {
    let mut x = 4;
    4;
    print!("{}{}", 4, 4);
}
*/