fn main() {
    let mut a = 5;
    let mut b = 3;
    print!("{}", a-- - --b);
}
/*
2
Unlike C or Java, there is no unary increment or decrement operator in Rust. The Rust language design FAQ (no longer available online) used to touch on the reason:

Why doesn't Rust have increment and decrement operators?
Preincrement and postincrement (and the decrement equivalents), while convenient, are also fairly complex. They require knowledge of evaluation order, and often lead to subtle bugs and undefined behavior in C and C++. x = x + 1 or x += 1 is only slightly longer, but unambiguous.

In the absense of postfix and prefix decrement operators, a-- - --b is parsed as a - (-(-(-(-b)))). In the case of a = 5 and b = 3 the value of this expression is 5 - 3 which is 2.
*/