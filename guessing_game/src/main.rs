use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // 产生一个1-100间的随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {}", secret_number);

    // 放到大循环
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // expect 处理异常
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
        // match 处理异常 parse() 返回Result枚举
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        // 比较
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {  // 中断
                println!("You win!");
                break;
            }
        }
    }
}
