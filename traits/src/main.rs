use core::fmt::{Display, Debug};

fn main() {
    println!("Traits!");
    traits();
}

pub trait Summary {
    // Default summary if not implemented.
    fn summarize(&self) -> String {
       String::from("Some default summary!") 
    }

    // Required to be filled out
    fn author(&self) -> String;
}

pub fn notify<T: Summary + Display>(item: T) {
    println!("summary={}", item.summarize());
}

pub fn some_function <T, U>(t: T, u: U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
    23
}

pub struct NewsArticle {
    pub headline: String,
    pub content: String,
}

// Use the default case for summary.
impl Summary for NewsArticle {
    fn author(&self) -> String {
        String::from("N/A")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} -- {}", self.username, self.content)
    }

    fn author(&self) -> String {
        format!("{}", self.username)
    }
}

fn traits() {
    let tweet = Tweet {
        username: "Jonathan".to_string(),
        content: String::from("Some content"),
    };
    println!("tweet.summarize(): {}", tweet.summarize());

    let article = NewsArticle {
        headline: "Some headline!".to_string(),
        content: String::from("some news content"),
    };
    println!("article.summerize(): {}", article.summarize());
}
