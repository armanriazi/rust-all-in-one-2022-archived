fn main() {
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);
    //
    let  number=3;
    println!("Number {}", number);
    {
        let number = 5;
        println!("Number {}",number);
    }
    println!("Number {}",number);
}