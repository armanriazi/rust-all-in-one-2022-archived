fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn main() {
    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b)= swap(result.0, result.1);
    let (z, y)= swap(a, b);
    println!("{} {}", a, b);
    println!("{} {}", z, y);
}