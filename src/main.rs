fn main() {
        let reference_to_nothing = dangle();
    //let reference_to_nothing = no_dangle();
    }
    fn dangle() -> &'static i32 { // dangle returns a reference to a String
    
        let s:&'static =10;
    
        s // we return a reference to the String, s
    } // Here, s goes out of scope, and is dropped. Its memory goes away.
      // Danger!
    
      fn no_dangle() -> i32 {
        let s = 100;
    
        s
    }