fn main() {
    println!("Hello, world!");
    another_function(9);

    let x = five();
    let x = plus_one(x);
    println!("The value of x is: {}", x);

}

fn another_function(x: i32) {
    
    let y = {
        let x = 3;
        x + 1  // Expressions do not include ending semicolons
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}