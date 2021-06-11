use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);  // field_name, field_value不能再使用

    // vec of tuple to hash map use zip
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    // get 取值
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("{}", score.unwrap());
    // 遍历
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // update
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);  // 替换旧的

    // entry 如果有值不改变，如果没值改变值
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // returns a mutable reference (&mut V)
        *count += 1;
    }

    println!("{:?}", map);

    

}
