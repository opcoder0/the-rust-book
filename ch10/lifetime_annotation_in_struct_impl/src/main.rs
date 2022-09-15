#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn version(&self) -> i32 {
        1
    }

    fn announce_and_return_part(&self, announce: &str) -> &str {
        println!("Annoucement: {}", announce);
        self.part
    }
}

fn main() {
    let news = "NLL work will be enabled by default. NLL is the second iteration of Rust's borrow checker. The RFC actually does quite a nice job of highlighting some of the motivating examples.";
    let first_sentence = news
        .split('.')
        .next()
        .expect("The string does not have a dot(.)");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!(
        "Important Excerpt: Version {}: {:?}",
        i.version(),
        i.announce_and_return_part(news)
    );
}
