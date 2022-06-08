
// #![allow(unused)]
// fn main() {
// trait Shape { fn area(&self) -> f64; }
// trait Circle : Shape { fn radius(&self) -> f64; }
// }

// #![allow(unused)]
// fn main() {
// trait Shape { fn area(&self) -> f64; }
// trait Circle where Self: Shape { fn radius(&self) -> f64; }
// }

// #![allow(unused)]
// fn main() {
// trait Shape { fn area(&self) -> f64; }
// trait Circle where Self: Shape {
//     fn radius(&self) -> f64 {
//         // A = pi * r^2
//         // so algebraically,
//         // r = sqrt(A / pi)
//         (self.area() /std::f64::consts::PI).sqrt()
//     }
// }
// }



#![allow(unused)]
fn main() {
trait Shape { fn area(&self) -> f64; }
trait Circle : Shape { fn radius(&self) -> f64; }
fn print_area_and_radius<C: Circle>(c: C) {
    // Here we call the area method from the supertrait `Shape` of `Circle`.
    println!("Area: {}", c.area());
    println!("Radius: {}", c.radius());
}
}
