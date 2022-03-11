use std::io;
use std::process;
fn main (){
loop{
    let mut first= String::new();
    
    println!("Enter first number:");
    io::stdin().read_line(&mut first);
    //let a:u32 = first.trim().parse().expect("This is not valid number");//unwrap();
    //unwrap: if this result does not emit an error. it will resolve with the value
    
    let mut a:u32 = 0;
    match first.trim().parse(){
        Ok(val)=>{
            a=val;
        },
        Err(_err)=>{
            println!("This is not a valid number");
            process::exit(1);
        }
    }
    let mut second= String::new();
    println!("Enter second number:");
    io::stdin().read_line(&mut second);
    let b:u32 = second.trim().parse().expect("This is not valid number");

    let result = sum(a,b);
    println!("{} + {} = {}", a,b,result);
 }   
}

fn sum(a:u32, b:u32) -> u32{
    a+b
}