use std::io;

fn main() {
    // Scalar
    // integers, floating-point numbers, Booleans, and characters

    // integers
    let _a: u8 = 128;  // i8
    let _a = 98_222;  // Decimal
    let _a = 0xff;  // Hex
    let _a = 0o77;  // Octal
    let _a = 0b1111_0000; // Binary
    let _a = b'A';  // Byte (u8 only)
    println!("a: {}", _a);

    // floating-point numbers
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Boolean
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    // characters
    // char type is four bytes in size and represents a Unicode Scalar Value
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';


    // Compound Types: tuples and arrays
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    // a tuple element directly by using a period (.) followed by the index of the value
    let five_hundred = tup.0;
    let six_point_four = tup.1;

    // Array
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let first = a[0];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );

}
