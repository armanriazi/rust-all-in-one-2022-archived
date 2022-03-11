 #![allow(unused_variables)]
   
  #[derive(Debug)]
  //Implementing the short-lived variables strategy
  //some sat
  struct CubeSat {
    id: u64,
  }
   
  #[derive(Debug)]
  struct Mailbox {
  messages: Vec<Message>,
}
 
#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
    
}
 
struct GroundStation {}
 
impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }
 
    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            //That way our now-proxy CubeSat instances can match their IDs to receive messages:
            if self.messages[i].to == recipient.id {
                /*Astute readers of listing 4.14 will notice a strong anti-pattern. On line 32, the self.messages collection is modified while it is being iterated over. In this instance, this is legal because of the return on the next line. The compiler can prove that another iteration will not occur and allows the mutation to proceed. */
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
 
        None
    }
}
 
impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat {
            id: sat_id,
        }
    }
 
    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}
 
impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

fn fetch_sat_ids() -> Vec<u64> {
  vec![1,2,3]
}
 
 
fn main() {
  let mut mail = Mailbox { messages: vec![] };
 
  let base = GroundStation {};
 
  let sat_ids = fetch_sat_ids();
 
  for sat_id in sat_ids {
    let sat = base.connect(sat_id);
    let msg = Message { to: sat_id, content: String::from("hello") };
    base.send(&mut mail, msg);
  }
 
  let sat_ids = fetch_sat_ids();
 
  for sat_id in sat_ids {
    let sat = base.connect(sat_id);
 
    let msg = sat.recv(&mut mail);
    println!("{:?}: {:?}", sat, msg);
  }
}