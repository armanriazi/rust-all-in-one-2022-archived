///#Experimental non-stable channel build
///![feature(lint_reasons)]
///![allow(clippy::some_lint, reason = "False positive rust-lang/rust-clippy#1002020")]
/// #

```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```