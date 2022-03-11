fn main(){
    let mut res=42;
    let option = Some(12);
    if let Some(x)=option{
        res+=x;
    }
    /* no clippy code
    for x in option{
        res+=x;
    }*/
    println!("Printed:{}",res);
    
}