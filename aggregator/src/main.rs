use ::aggregator::{NewsArticle, SocialPost, Summary};

fn main() {
    let article = NewsArticle {
        headline: String::from("The Daily Rust News"),
        location: String::from("New York"),
        author: String::from("Edsger W. Dikker"),
        content: String::from(
            "Rust is a system programming language focused on safety, speed and concurrency.",
        ),
    };

    println!("New article available! {}", article.summarize());

    let post = SocialPost {
        username: String::from("*social.rust-lang.org*"),
        content: String::from("Rust 1.97.0 has been released! "),
        reply: false,
        repost: false,
    };

    println!("1 new post:{}", post.summarize())
}
