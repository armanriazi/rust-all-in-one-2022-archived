use std::fs::File;

fn main() {
    let f = File::open("hello1.txt").expect("Failed to open hello.txt");
}
