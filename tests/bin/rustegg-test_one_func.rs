fn calc_apple_price(num:i32) -> i32 {
    //if num.is_numeric() //is_alphabetic
    //{  }else{_}
        if num > 40{
            num
        }
        else{
            num+25
        }
  
}

fn main(){

}

#[test]
fn verify_test(){
    let price1=calc_apple_price(35);
    let price2=calc_apple_price(65);
    assert_eq!(60,price1);
    assert_eq!(65,price2);
}