fn main(){
    let x = 1.2331f64;
    let y   = 1.2332f64;
    let error = 0.00001f64;
    println!("Printed:{}",x-y);
    
    if(x-y).abs() <error{
        println!("success!");
    }
}