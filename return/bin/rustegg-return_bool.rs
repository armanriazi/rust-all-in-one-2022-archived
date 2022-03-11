fn main(){

    let word=String::from("green");
    if iscolor(&word){
        println!("that is color");        
    }
    else{
        println!("that is no color");        
    }
}

fn iscolor(attempt: &str)-> bool{
attempt == "green" || attempt == "red" || attempt=="blue"
}
