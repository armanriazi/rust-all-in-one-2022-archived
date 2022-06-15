//#![feature(fn_traits)]
#![feature(unboxed_closures)]
//#[cfg(feature = "unboxed_closures")]

struct Closure<'a> {
    s : String,
    t : &'a String,
}

// impl FnMut<()> for Closure {
//     type Output = String;
//     extern "rust-call"  fn call_mut(args: Closure) -> Self::Output {
//         println!("call_mut()");
//         args.s += &*args.t;
//         args.s
//     }
// }
impl<'a>  FnOnce<()> for Closure <'a> {
    type Output = String;
    extern "rust-call"  fn call_once(args: Closure) -> Self::Output {
       //println!("Printed:{:?}",args);
       args.s
    }
}


// extern "rust-call" fn add_args(args: (u32, u32)) -> u32 {
//     args.0 + args.1
// }

// struct Cre(u32, u32);
// fn main() {
   
//     //let a=Cre(10,5);
//     println!("Printed:{:?}",add_args((10,5)));
    
    
// }


fn main(){
    let mut s = String::from("foo");
    let t = String::from("bar");
    let mut f=Closure::<'a>{s: s, t: &'a t}
    
    //let mut ww: Box<dyn FnOnce()> = Box::new(f)
    //let output=ww();
    println!("call_count is {}", f.call_once(&f));
}