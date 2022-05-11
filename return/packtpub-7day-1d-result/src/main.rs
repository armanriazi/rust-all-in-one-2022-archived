use std::collections::HashMap;
use std::env::args;



// cargo run  arg1 arg2=sth arg3


//args:usize
fn main() {

    let mut hm = HashMap::new();

    hm.insert(3 ,"Hello");
    hm.insert(5, "world");

    let r = hm.get(&4).unwrap_or(&"NoString"); 

    println!("{}", r);

    match get_arg(3) {
        Ok(v)=>println!("OK - {}", v),
        Err(e)=>println!("Errrorr - {}",e),
    }
}


fn get_arg(n:usize)->Result<String,String>{
    //yields pairs (i, val)
    for (i,a) in args().enumerate(){
        if i == n{
            return Ok(a);
        }
    }
    Err("Not enough Args".to_string())
}







