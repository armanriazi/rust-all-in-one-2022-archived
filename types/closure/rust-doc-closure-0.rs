fn f<F : FnOnce() -> String> (g: F) {
    println!("{}", g());
}


fn main(){
let mut s = String::from("foo");
let t = String::from("bar");

f(|| {
    s += &t;
    s
});
}
// Prints "foobar".