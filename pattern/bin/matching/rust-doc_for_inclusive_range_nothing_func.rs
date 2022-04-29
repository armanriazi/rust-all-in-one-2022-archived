fn nothing()->(){

}

fn main() {
    for x in 0..5 {
        println!("{}", x);
    }
    
    let ()= nothing();

    println!("{:?}", ());

    for x in 0..=5 {
        println!("{}", x);
    }

    for z in 8..=13 {
        println!("{}", z);
    }
}