fn main()
{
    let x = -12; // by default this is i32
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    let z: u16=13;
    const PI: f32 = 3.14159;
    println!(
        "{} {} {} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence, z, PI
    );
}