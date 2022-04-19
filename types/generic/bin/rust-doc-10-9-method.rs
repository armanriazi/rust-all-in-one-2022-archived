struct Point<T> 
where T:PartialOrd {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        if (&self.x == 5)
        0
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}