macro_rules! string {
    ($x:expr) => ( // <1>
        String::from(stringify!($x)) // <2>
    )
}