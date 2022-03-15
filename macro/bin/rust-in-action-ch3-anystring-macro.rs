macro_rules! MyString {
    ($x:expr) => ( // <1>
        String::from(stringify!($x)); // <2>
    )
}

fn main() {
    let s = MyString!(hello there);
    println!("{}", s);
}