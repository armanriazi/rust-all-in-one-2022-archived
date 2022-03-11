use num::complex::Complex;    // <1>

fn main() {
  let a = Complex { re: 2.1, im: -1.2 };  // <2>
  let b = Complex::new(11.1, 22.2);        // <3>
  let result = a + b;
  
  println!("{} + {}i", result.re, result.im)
}