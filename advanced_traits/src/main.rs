
use std::ops::Add;

// Specifying Placeholder Types in Trait Definitions with Associated Types
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {}
// can’t implement a trait on a type multiple times

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(1)
    }
}

// Default Generic Type Parameters and Operator Overloading
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implementing the Add trait on Millimeters to add Millimeters to Meters
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

// Two traits are defined to have a fly method and are implemented on the Human type, and a fly method is implemented on Human directly
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

// Using the Newtype Pattern to Implement External Traits on External Types
use std::fmt;

// make a Wrapper struct that holds an instance of Vec<T>
struct Wrapper(Vec<String>);
// uses self.0 to access the inner Vec<T>
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
// The Never Type that Never Returns
fn bar() -> ! {
    println!()
}

// pass regular functions to functions Function Pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// 
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    // default the fly method implemented on Human directly
    person.fly();
    // Specifying which trait’s fly method we want to call
    Pilot::fly(&person);
    Wizard::fly(&person);

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);


    // Creating Type Synonyms with Type Aliases
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Function Pointers
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
