fn main() {
    let x = 90;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        
        
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        90 | 101 => {
            println!("found  number between 90 | 101!");
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }
}