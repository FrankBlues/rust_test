fn main() {
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // // 
    // let condition = true;
    // let number = if condition {5} else {6};

    // println!("The value of number is: {}", number);

    // // Returning Values from Loops
    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {}", result);

    // // while loop
    // let mut number = 3;

    // while number != 0 {
    //     println!("{}!", number);
    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // // for loop
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }
    // // for
    // for element in a.iter() {
    //     println!("the value is: {}", element);
    // }
    // // range
    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    // fib 函数
    let n:u64 = 100;
    println!("The {}th number is {}.", n, fib(n));
}

fn fib(n: u64) -> u128{
    if n == 1 || n == 2 {
        1
    } else {
        let mut i =3;
        let mut x: u128 = 1;
        let mut y: u128 = 1;
        loop {
            let tmp = y;
            y = x + y;
            x = tmp;

            if i == n {
                break y;
            }
            i += 1;
        }
    }
}