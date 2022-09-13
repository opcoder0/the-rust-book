use aggregator::{NewsArticle, Summary, Tweet};

mod aggregator;

fn main() {
    let tweet = Tweet {
        username: String::from("tcm"),
        content: String::from("era of abundance... is it really?"),
        retweet: false,
        reply: false,
    };
    println!("Tweet Summary: {}", tweet.summarize());

    let news = NewsArticle {
        author: String::from("the common man"),
        location: String::from("news paper"),
        content: String::from("era of abundance... is it really?"),
        headline: String::from("era of abundance... is it really?"),
    };
    println!("NewsArticle Summary (default impl): {}", news.summarize());
}
