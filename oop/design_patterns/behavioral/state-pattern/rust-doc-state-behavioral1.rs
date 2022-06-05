//! State is a behavioral design pattern that lets an object alter its behavior when its internal state changes.
//! It appears as if the object changed its class.

//! We’ll implement a blog post workflow
//! 1. A blog post starts as an empty draft.
//! 2. When the draft is done, a review of the post is requested.
//! 3. When the post is approved, it gets published.
//! 4. Only published blog posts return content to print, so unapproved posts can’t accidentally be published.

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}

/*
If we were to create an alternative implementation that didn’t use the state pattern, we might instead use match expressions in the methods on Post or even in the main code that checks the state of the post and changes behavior in those places. That would mean we would have to look in several places to understand all the implications of a post being in the published state! This would only increase the more states we added: each of those match expressions would need another arm.

With the state pattern, the Post methods and the places we use Post don’t need match expressions, and to add a new state, we would only need to add a new struct and implement the trait methods on that one struct.
*/
struct Draft;
struct PendingReview;
struct Published;


// 1 internal state changes.

impl State for Draft {
    fn request_review(self: Box<Draft>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
    fn approve(self: Box<Draft>) -> Box<dyn State> {
        self
    }
}

// 2 internal state changes.
impl State for PendingReview {
    fn request_review(self: Box<PendingReview>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<PendingReview>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// 3 internal state changes.

impl State for Published {
    fn request_review(self: Box<Published>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Published>) -> Box<dyn State> {
        self
    }
    //Important
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    //Because state is an Option<Box<dyn State>>, when we call as_ref, an Option<&Box<dyn State>> is returned
    // This func will check in every steps
    fn content(&self) -> &str {        
        self.state.as_ref().unwrap().content(self)
    }
    fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

fn main() {
    //Make Draff
    let mut post = Post::new();
  
    let text = "State is a behavioral design pattern.";
    post.add_text(text);
    assert_eq!("", post.content());
    //Make PendingReview
    post.request_review();
    assert_eq!("", post.content());
    //Make Published
    post.approve();
    assert_eq!(text, post.content());
    println!("post content: {}", post.content());
    /* Related *** bookmark 
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!(text, post.content());
    println!("post content: {}", post.content());
    */
}


/*
Follow Duplication methods 
Other duplication includes the similar implementations of the request_review and approve methods on Post. Both methods delegate to the implementation of the same method on the value in the state field of Option and set the new value of the state field to the result. If we had a lot of methods on Post that followed this pattern, we might consider defining a macro to eliminate the repetition
*/

/*
Future work:
try a few of these suggestions:

Add a reject method that changes the post’s state from PendingReview back to Draft.
Require two calls to approve before the state can be changed to Published.
Allow users to add text content only when a post is in the Draft state. Hint: have the state object responsible for what might change about the content but not responsible for modifying the Post.

Related *** bookmark: One downside of the state pattern is that, because the states implement the transitions between states, some of the states are coupled to each other. If we add another state between PendingReview and Published, such as Scheduled, we would have to change the code in PendingReview to transition to Scheduled instead. It would be less work if PendingReview didn’t need to change with the addition of a new state, but that would mean switching to another design pattern.
*/