// In return because we do not have Type like String so we can dangling problem
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let a=first_word("Arman Riazi");
    println!("Printed:{:?}",a);
    
}