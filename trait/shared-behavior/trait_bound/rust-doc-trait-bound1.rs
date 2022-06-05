fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
unimplemented()
}
//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {