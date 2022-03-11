fn main(){
    let a = 0..100;
    println!("{:?}",a);
    
    if a.len() >= 100{
        println!("Big");
    }
    else{
        println!("Short");
    }
}