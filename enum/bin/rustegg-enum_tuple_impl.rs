#[derive(Debug)]
enum Message {
    //classic : c-like structures.
    Move{x:u32,y:u32},
    Echo(String),
    ChangeColor(u8,u8,u8),
    //`unit-like`,
    Quit
}

impl Message {
    fn call(&self){
        println!("{:?}",&self);
    }
}

fn main(){
    let messages = [
        Message::Move{ x: 10, y:30},
        Message::Echo(String::from("hello")),
        Message::ChangeColor(200,255,255),
        Message::Quit
    ];

    for msg in & messages {
        msg.call();
    }
}