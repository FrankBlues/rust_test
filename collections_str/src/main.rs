fn main() {
    // String
    // init
    let mut s = String::new();

    let data = "initial contents";

    let s = data.to_string();

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    let s = String::from("initial contents");
    let hello = String::from("你好");

    // update
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);  // donot take ownership
    println!("s2 is {}", s2);

    s1.push('l'); //char

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s = s1 + "-" + &s2 + "-" + &s3;
    let s = format!("{}-{}-{}", s1, s2, s3);  // donot take ownership

    // slice
    // let s0 = &s[0];  // error 因为utf8 cannot index
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // 用slice 返回2个字符,因为一个字符2个字节
    println!("{}", s);

    // .chars 返回每个字符
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // .bytes返回每个字节
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
}

