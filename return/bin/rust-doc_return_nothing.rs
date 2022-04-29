
fn make_nothing() -> () {
    return ();
}

// the return type is implied as ()
fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn main() {
    let a = make_nothing();
    let b = make_nothing2();
    let c = 8.99f64;
    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);
    println!("The value of c: {:?}", c);
}