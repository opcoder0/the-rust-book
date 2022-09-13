use aggregator::{NewsArticle, Summary, Tweet};

mod aggregator;

fn notify<T: Summary>(item: &T) {
    println!("[Breaking News]: {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("tcm"),
        content: String::from("era of abundance... is it really?"),
        retweet: false,
        reply: false,
    };
    println!("Tweet Summary: {}", tweet.summarize());
    notify(&tweet);

    let news = NewsArticle {
        author: String::from("the common man"),
        location: String::from("news paper"),
        content: String::from("era of abundance... is it really?"),
        headline: String::from("era of abundance... is it really?"),
    };
    println!("NewsArticle Summary (default impl): {}", news.summarize());
    notify(&news);
}
