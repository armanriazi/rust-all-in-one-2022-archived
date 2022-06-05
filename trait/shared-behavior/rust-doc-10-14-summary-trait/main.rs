
use aggregator::{Summary, Tweet};
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
