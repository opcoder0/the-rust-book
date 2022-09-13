use aggregator::{NewsArticle, Summary, Tweet};

mod aggregator;

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
