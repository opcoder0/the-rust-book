pub trait Summary {
    fn summarize(&self) -> String;
}

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
    pub retweet: bool,
    pub reply: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main() {
    let article: NewsArticle = NewsArticle {
        author: String::from("the common man"),
        location: String::from("tcm@gmail.com"),
        headline: String::from("era of abundance.. is it really?"),
        content: String::from("... on page ..."),
    };

    println!("article summary: {}", article.summarize());

    let tweet: Tweet = Tweet {
        username: String::from("tcm"),
        content: String::from("era of abundance... is it really?"),
        retweet: false,
        reply: false,
    };
    println!("tweet summary: {}", tweet.summarize());
}
