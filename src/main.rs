use std::{
    fmt::{Debug, Display},
    io,
};
mod re;
mod store;
mod user;

struct NewsArticle {
    author: String,
    headline: String,
    content: String,
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x, y }
    }
}

impl<T: num::Float + Display> Point<T> {
    pub fn print_rounded(&self) {
        println!("x: {:.2}", self.x);
        println!("y: {:.2}", self.y);
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("(read less from {})", self.summarize_author())
    }
    fn summarize_author(&self) -> String {
        format!("The best author in the fucking world aka {}", self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("The worst author in the fucking world is {}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

trait Summary {
    // has a default implementation
    // so it doesn't have to be implemented
    // on types that implement this trait
    // but it's possible to do so, in which case
    // the default implementation gets overridden
    fn summarize(&self) -> String {
        format!("(read more from {}...)", self.summarize_author())
    }

    // does not have a default implementation,
    // so it needs to be implemented in every type
    // that implements this trait
    fn summarize_author(&self) -> String;
}

fn soy_virgin_notify<T: Summary>(news: &T) {
    for _ in 0..10 {
        println!("BREAKING NEWS!!!!!");
    }
    println!("The news: {}", news.summarize());
    for _ in 0..10 {
        println!("BREAKING NEWS!!!!!");
    }
}

fn notify1<T: Summary>(item1: &T, item2: &T) {
    println!("2 breaking newses of the same type:");
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

fn notify2<T: Summary, U: Summary>(item1: &T, item2: &T, item3: &U) {
    println!("2 breaking newses of the same type:");
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
    println!("1 breaking new of a potentially different type:");
    println!("{}", item3.summarize());
}

fn notify3(item1: &(impl Summary + Display), item2: &impl Summary) {
    println!("2 breaking newses of potentially different types:");
    println!("{item1}");
    println!("{}", item1.summarize());
    println!("{}", item2.summarize());
}

// fn some_function<T: Clone + Display, U: Clone + Debug>(item1: &T, item2: &U) {
//     todo!("do some work here");
// }

fn some_function<T, U>(item1: T, item2: U)
where
    T: Clone + Debug,
    U: Clone + Summary,
{
    todo!("do some work here");
}

fn main() -> Result<(), io::Error> {
    let p1 = Point::<f64>::new(2345.6646, 20.0);
    p1.print_rounded();

    let article = NewsArticle {
        author: String::from("@the_guy_420"),
        headline: String::from("The sky is falling"),
        content: String::from("i swear it's real"),
    };

    let tweet = Tweet {
        username: String::from("@cool_mad_scientist"),
        content: String::from("The sky is NOT falling!!! It's all fake fucking news!!!"),
        reply: true,
        retweet: false,
    };
    let tweet2 = Tweet {
        username: String::from("@cool_mad_scientist"),
        content: String::from("The sky is NOT falling!!! It's all fake fucking news!!!"),
        reply: true,
        retweet: false,
    };

    soy_virgin_notify(&tweet);
    notify1(&tweet, &tweet2);
    notify2(&article, &article, &article);
    notify2(&article, &article, &tweet);
    notify3(&tweet, &article);

    Ok(())
}
