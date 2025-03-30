use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String;
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify1(item: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify2<T: Summary>(item: &T, item2: &T) {
    println!("Breaking news! {}", item.summarize())
}

pub fn notify3<T: Summary + Debug>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}