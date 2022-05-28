fn main() {
    
    use std::collections::HashMap;

    let mut map = HashMap::<String, String>::default();
    map.insert("hello?".into(), "Hello!".into());

    println!("{:?}", map.get("hello?"));
}
