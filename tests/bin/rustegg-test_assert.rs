fn is_even(num:i32)-> bool{
    num%2==0
}
pub fn times_two(num:i32)-> i32{
    num*2
}
fn main(){

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even(){
            //assert!(false);//disable all assert
            assert!(is_even(2));
    }
    #[test]
    fn is_false_when_odd(){
        /*assert!(is_even(7));        
        assert!(true);
        assert!(is_even(6));*/
        assert_eq!(false, is_even(3));

    }
    #[test]
    fn ret_twice_of_positive_numbers(){
        assert_eq!(times_two(4),8);        
    }
    #[test]
    fn ret_twice_of_negative_numbers(){
        assert_eq!(times_two(-4),-8);        
    }
}