fn consume_with_relish<F>(func: F)
    where F: FnOnce() -> String
{
    // `func` consumes its captured variables, so it cannot be run more
    // than once.
    println!("Consumed: {}", func());

    println!("Delicious!");

    //println!("Consumed2th: {}", func());

    // Attempting to invoke `func()` again will throw a `use of moved
    // value` error for `func`.
}
fn main(){
    let x = String::from("x");
    let consume_and_return_x = move || x;
    consume_with_relish(consume_and_return_x);
}