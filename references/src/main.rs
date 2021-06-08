fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // 用引用 否则s1 moved to function,后面不能用
    println!("The length of '{}' is {}.", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);  // 通过引用改变参数
    println!("Changed s2: {}", s2);

    // 只能同时有一个可变的引用
    let mut s3 = String::from("hello");
    {let r1 = &mut s3;}
    let r2 = &mut s3;

    let r1 = &s3; // no problem
    let r2 = &s3; // no problem
    // let r3 = &mut s3; // BIG PROBLEM 已有非可变引用，再有可变引用会冲突报错

    println!("{}, {}, and", r1, r2);

    // 
    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value 5
    println!("{}", word);
    s.clear(); // this empties the String, making it equal to ""
    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    // 局部引用字符串
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word_str(&s[..]);
    println!("{}", word);
    let h = &s;
    // let i = s;
    println!("{}", hello);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// str: string slices
fn first_word_str(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}