// TO_REPORT_BUG: *BecauseOf(-): *Help(+):rust•armanriazi•error•value used here after move *BecauseOf(?):- 

#[derive(Debug)]
 struct CubeSat {
   id: u64,
 }
 
 #[derive(Debug)]
 enum StatusMessage {
   Ok,
 }
 
 /*fn check_status(
   sat_id: CubeSat
 ) -> StatusMessage {
     //implicit code:  drop(sat_a)
   StatusMessage::Ok
 }*/
 
 fn check_status(sat_id: CubeSat) -> CubeSat {
 
    println!("{:?}: {:?}", sat_id,
                           StatusMessage::Ok);
    sat_id
   
  }
 
 fn main() {
   let sat_a = CubeSat { id: 0 };//we can see that sat_a starts its life with ownership over a CubeSat object:
   let sat_b = CubeSat { id: 1 };
   let sat_c = CubeSat { id: 2 };
 
   let a_status = check_status(sat_a);
   let b_status = check_status(sat_b);
   let c_status = check_status(sat_c);
   println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
 
   // "waiting" ...
   let a_status = check_status(sat_a);
   let b_status = check_status(sat_b);
   let c_status = check_status(sat_c);
   println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
 }