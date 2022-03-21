use std::{thread, time};
  
 fn main() {
   let start = time::Instant::now();
  
   let handler_1 = thread::spawn(move || {
     let pause = time::Duration::from_millis(300);
       thread::sleep(pause.clone());
   });
  
   let handler_2 = thread::spawn(move || {
     let pause = time::Duration::from_millis(300);
       thread::sleep(pause.clone());
   });
  
   handler_1.join().unwrap();
   handler_2.join().unwrap();
  
   let finish = time::Instant::now();
  
   println!("{:?}", finish.duration_since(start));
 }