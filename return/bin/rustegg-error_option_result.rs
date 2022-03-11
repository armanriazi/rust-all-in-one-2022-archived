pub fn generate_nametag_text(name: String) -> Result<String,String> {    

    if name.len() > 0 {
            Ok(format!("Hi! My name is {}", name))
            }
            else {
                Err(String::from("'name' was empty; it must be noneempty"))
            }
}

pub fn opt_generate_nametag_text(name: String) -> Option<String> {    

    if name.len() > 0 {
            Some(format!("Hi! My name is {}", name))
            }
            else {
                None
            }
}

fn main(){

}

#[cfg(test)]
mod tests {
use super::*;

#[test]
fn generate_name_tag_text_for_nonempty_name(){
    assert_eq!(
        generate_nametag_text("Alizay".into()),
        Ok("Hi! My name is Alizay".into())
        );
    }
#[test]
    fn generate_name_tag_text_for_nonempty_fails(){
        assert_eq!(
            generate_nametag_text("".into()),
            Err("'name' was empty; it must be noneempty".into())
            );
    }


#[test]
fn opt_generate_name_tag_text_for_nonempty_name(){
    assert_eq!(
        opt_generate_nametag_text("Alizay".into()),
        Some("Hi! My name is Alizay".into())
        );
    }
#[test]
    fn opt_generate_name_tag_text_for_nonempty_fails(){
        assert_eq!(
            opt_generate_nametag_text("".into()),
            None
            );
    }
}