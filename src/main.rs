trait Summary {
    fn summarize(&self) -> String;
}

trait Firend {
    fn add_friend(&self) -> String;
}

struct Person {
    name: String,
    age: u16,
}

impl Firend for Person {
    fn add_friend(&self) -> String {
        return "I added you as my Homie Friend".to_string();
    }
}

struct NewsArticle {
    headling: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        return "News Article summary of the content".to_string();
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: Option<bool>,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = &self.content;
        return "Tweet summary of the content".to_string() + " - " + content;
    }
}

fn notify<T: Summary, F: Firend>(item: T, friend: F) {
    friend.add_friend();
    println!("Summary is: {}", item.summarize())
}

fn main() {
    let tweet = Tweet {
        username: "ahmed09".to_string(),
        content: "tweet content".to_string(),
        reply: true,
        retweet: None,
    };

    let person = Person {
        name: "Zainab Ahmed Ragab".to_string(),
        age: 10,
    };

    println!("{}", tweet.summarize());
    notify(tweet, person);
}
