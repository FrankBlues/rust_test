
// define a trait named Draw
pub trait Draw {
    fn draw(&self);
}

// Definition of the Screen struct with a components field holding a vector
// of trait objects that implement the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// will call the draw method on each of its components
// trait objects allow for multiple concrete types to fill in for the trait object at runtime
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// An alternate implementation of the Screen struct and its run method using generics and trait bounds
// A generic type parameter can only be substituted with one concrete type at a time
// pub struct Screen1<T: Draw> {
//     pub components: Vec<T>,
// }

// impl<T> Screen1<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
        println!("This is a button.");
    }
}

