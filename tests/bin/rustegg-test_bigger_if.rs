pub fn bigger(a: i32,b: i32) -> i32{
    if a>b{
    a
    }
    else{
    b
    }

}
fn main(){

}

 #[cfg(test)]
 mod tests {
     use super::*;

     #[test]
     fn ten_is_bigger_than_eight(){
         assert_eq!(10, bigger(10,8));         
     }

     #[test]
     fn forthy_is_bigger_than_thirtytwo(){
         assert_eq!(42, bigger(32,42));         
     }
 }