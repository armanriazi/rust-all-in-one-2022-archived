

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}



pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

//use aggregator::{Summary, Tweet};

//**pub fn notify(item: &(impl Summary + Display)) {
//**pub fn notify<T: Summary + Display>(item: &T) {
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}