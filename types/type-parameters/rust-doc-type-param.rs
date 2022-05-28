#![allow(unused)]
fn to_vec<A: Clone>(xs: &[A]) -> Vec<A> {
    if xs.is_empty() {
        return vec![];
    }
    let first: A = xs[0].clone();
    let mut rest: Vec<A> = to_vec(&xs[1..]);
    rest.insert(0, first);
    //println!("{:?}",&res);
    rest
}

fn main() {
    let v=vec![1,2,3];
    let res=to_vec(&v[..]);
    println!("{:?}",&res);
    
}