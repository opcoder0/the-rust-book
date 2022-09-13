pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more...)")
    }
}

pub struct NewsArticle {
    pub author: String,
    pub headline: String,
    pub content: String,
    pub location: String,
}

impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
