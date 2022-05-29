

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
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

impl Summary for NewsArticle {
   

    fn summarize_author(&self) -> String {
        format!("@{}", self.headline)
    }
}

//use aggregator::{Summary, Tweet};

//**fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 { */
/* 
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
*/
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

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    notify(&tweet);
    println!("New article available! {}", article.summarize());
}