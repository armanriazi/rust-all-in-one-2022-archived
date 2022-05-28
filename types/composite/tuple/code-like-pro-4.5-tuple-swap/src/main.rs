fn swap<A, B>(a: A, b: B) -> (B, A) {
    (b, a)
}

fn main() {
    let a = 1f64;
    let b = 2i32;

    println!("{:?}", swap(a, b));
}
