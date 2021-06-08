fn main() {
    let width1 = 30.0;
    let height1 = 50.0;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // Use tuples
    let rec1 = (30.0, 50.0);
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(rec1)
    );

    // Struct
    let rect2 = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(&rect2)
    );

    println!("rect2 is {:#?}", rect2);
}

fn area(width: f64, height: f64) -> f64 {
    width * height
}

fn area1(dimensions: (f64, f64)) -> f64 {  // Use tuples
    dimensions.0 * dimensions.1
}

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn area2 (rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}