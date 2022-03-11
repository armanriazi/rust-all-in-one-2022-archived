
fn str_slice(arg: &str){
    println!("{}",arg);
}
fn mystr(arg : String){
    println!("{}",arg);
}

fn main(){
    
    str_slice("blue");
    str_slice(&String::from("abc")[0..2]);
    str_slice(" blue rt".trim());
    str_slice("red".into());

    mystr("red".to_string());
    mystr("red".to_owned());    
    mystr("Red".to_lowercase());
    mystr("Red Monday".to_string().replace("Mon","Tues"));
    mystr(format!("Hi{}","station"));

}

