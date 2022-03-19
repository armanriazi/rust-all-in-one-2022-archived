
fn str_slice(arg: &str){
    println!("{}",arg);
}
fn mystr(arg : String){
    println!("{}",arg);
}

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
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


    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string_slice("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());

}

