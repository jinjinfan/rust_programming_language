fn largest<T : PartialOrd + Copy>(list : &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
    
}

fn largest2<T : PartialOrd>(list : &[T]) -> &T {
    let mut largest = &list[0];
    let mut largest_idx = 0;
    for index in 0..list.len()-1{
        if list[index] > *largest {
            largest = &list[index];
            largest_idx = index;
        }
    }
    &list[largest_idx]
    
}

use std::fmt::Display;
use std::fmt::Debug;
/* Generic */
struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mixup<V, W>(self, other : Point<V, W>) -> Point<T, W> {
        Point {
            x : self.x,
            y : other.y
        }
    }
}

impl Point<f32, f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

/* traits */
pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline : String,
    pub location : String,
    pub author   : String,
    pub content  : String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("::{}", self.author)
    }
}

pub struct Tweet {
    pub username : String,
    pub content  : String,
    pub reply    : bool,
    pub retweet  : bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet{
    fn fmt(&self, f :&mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Username: {}", self.username)

    }
    
}

// pub fn notify<T: Summary + Display>(item : T)
pub fn notify(item:&(impl Summary + Display)) {
    print!("Breaking news! {}", item.summarize());
}

fn some_function<T,U>(t: T, u: U) -> i32
    where T : Display + Clone,
          U : Clone + Debug
{
    1
}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username : String::from("horse_ebooks"),
        content  : String::from("of course, as you probably already know, people"),
        reply    : false,
        retweet  : false,
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x:T, y:T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T:Display+PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// Life time for function
fn longest<'a>(x:&'a str, y:&'a str)->&'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime for struct holding reference
struct ImportantException<'a> {
    part: &'a str,
}

impl<'a> ImportantException<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement:&str) ->&str {
        println!("Attention please: {}", announcement);
        self.part
    }

}

fn longest_with_an_announcement<'a, T>(x:&'a str, y:&'a str, ann : T) ->&'a str
    where T:Display
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    // Largest
    let number_list = vec![34,50,56,100,25,65];
    println!("{}", largest2(&number_list));
    let char_list = vec!['y', 'm', 'a', 'q'];
    println!("{}", largest2(&char_list));
    
    // Generic
    let p = Point{x:5, y:10};
    println!("p.x = {}", p.x());
    // Traits
    let tweet = Tweet {
        username : String::from("horse_ebooks"),
        content  : String::from("of course, as you probably already know, people"),
        reply    : false,
        retweet  : false,
    };
    let arc = NewsArticle{
        headline : String::from("a"),
        location : String::from("b"),
        author   : String::from("c"),
        content  : String::from("d"),
    };
    let summaries = vec![tweet, arc];

    //notify(&tweet);

    // Lifetime annotation
    /*let novel = String::from("Call me.Some years ago...");
    let first_sentence = novel.split('.')
                        .next()
                        .expect("Could not find a '.'");
    let i = ImportantException{part:first_sentence};*/

}
