fn main() {
    let x = 5;
    let y = Box::new(x);// Box::new means '&'
    let y = Box::new(String::from("Arman"));
    let yy = String::from("Arman");
    
    let z= &(*y)[..];    
    let zz= &(*yy)[..];

    let o= &(y)[..];
    let oo= &(yy);
    println!("Printed:{:}",&oo);

    assert_eq!(5, x);
    assert_eq!("Arman", z);
    assert_eq!("Arman", zz);
    assert_eq!("Arman",o);
    assert_eq!("Arman",oo);
}
