#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// attach data to each variant of the enum directly
enum IpAddr1 {
    V4(String),
    V6(String),
}

// attach data of different type to each variant of the enum directly
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// enum whose variants each store different amounts and types of values
// but as a single type(Message)
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Call!")
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// match
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

// Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("Hello, world! {:#?}", four);

    let home = IpAddr1::V4(String::from("127.0.0.1"));
    let loopback = IpAddr1::V6(String::from("::1"));

    let home = IpAddr2::V4(127, 0, 0, 1);

    let m = Message::Write(String::from("hello"));
    let m = Message::Quit;
    m.call();

    let some_number = Some(5);  // Option enum  Some可以直接用
    let some_string = Some("a string");  // Option enum

    let absent_number: Option<i32> = None;  // Option enum None必须指明类型

    let c1 = Coin::Penny;
    let penny = value_in_cents(c1);

    let c2 = Coin::Quarter(UsState::Alabama);
    let quarter = value_in_cents(c2);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);

    // match 必须处理所有可能值, _可以代表剩余所有值
    let some_u8_value = 3u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }

    // 只想处理Some(3);用match需要用_占位符
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    // 可以用if let只处理一种情况
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let coin = &Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
