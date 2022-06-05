fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[..2];
    let slice2 = &s[3..];
    let slice3 = &s[..];
    println!("Printed:{:?}",slice3);
    

}