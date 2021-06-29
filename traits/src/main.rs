use std::fmt::{Display, Debug};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("New article available! {}", article.summarize());
}

// declare a trait using the trait keyword
// 类似接口
pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait Summary1 {
    // 默认行为
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

impl Summary1 for NewsArticle {}

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

// traits as parameters
// accepts any type that implements the specified trait
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//  equivalent
pub fn notify1<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// allow item1 and item2 to have different types
pub fn notify2(item1: &impl Summary, item2: &impl Summary) {}

//  force both parameters to have the same type
pub fn notify3<T: Summary>(item1: &T, item2: &T) {}

// more than one trait bound (+号)
pub fn notify4(item: &(impl Summary + Display)) {}

// 
pub fn notify5<T: Summary + Display>(item: &T) {}

// Using too many trait bounds
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {0}
// 用where的形式
fn some_function1<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{0}

// return a value of some type that implements a trait
// you can only use impl Trait if you’re returning a single type
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}