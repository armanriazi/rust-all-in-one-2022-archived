fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    println!("Printed:{} content{:?}",vec1.len(), vec1);    

    vec1.push(88);
    println!("Printed:{} content{:?}",vec1.len(), vec1);    
    
}

fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    let mut vec =vec![22,44,66];

    // vec.push(22);
    // vec.push(44);
    // vec.push(66);
    vec
}