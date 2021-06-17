use std::collections::HashMap;

fn main() {
    // vector
    let v: Vec<i32> = Vec::new();
    let mut v: Vec<i32> = Vec::new();  // 想改变内部值，必须指定为mut
    v.push(5);
    v.push(6);
    v.push(7);
    let v = vec![1, 2, 3, 4, 5];  // 用宏加初始值指定vector，自动推测类型

    // 用引用的方式取值，返回一个引用
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // 用get的方式取值，返回Option<&T>
    match v.get(2) { 
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // 超出范围
    //let a: &i32 = &v[100];  // 报错
    let b = v.get(100);  // 返回None
    println!("{:?}", b);

    for i in &v {  // 遍历vector
        println!("{}", i);
    }

    // 如果需要改变vector中的值 必须是mut
    let mut v = vec![1, 2, 3, 4, 5];
    for i in &mut v {
        if *i == 3 {  // 必须解引
            *i += 50;
        }
    }
    match v.get(2) {
        Some(s) => println!("{}", s),
        None => println!("None"),
    }
    // println!("{:?}", v.get(2));

    // 用enum包装多种类型,再放入一个vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut v = vec![3, 5, 10, 56, 78, 3, 20, 15, 30, 5, 78, 10, 12, 13];
    // let mut sum = 0;
    // for i in &v {
    //     sum += *i;
    // }
    println!("Mean of v: {}", mean_vec(&v));
    println!("Median of v: {}", median_vec(&mut v));
    println!("{:?}", v);

    let mode = mode_vec(&v);
    println!("The value that occurs most often: {:?}", &mode);
    // for i in &mode {
    //     println!("the value that occurs most often ")
    // }

    let s = String::from("egg");
    println!("{}", to_pig_latin(&s));

}

fn mean_vec (v: &Vec<i32>) -> f64 {
    // 均值
    let mut sum = 0;
    for i in v {
        sum += *i;
    }
    sum as f64 / v.len() as f64
}

fn median_vec (v: &mut Vec<i32>) -> i32 {
    // 中值 避免改变原值 先做复制
    let mut _v = v.clone();
    _v.sort_unstable();
    _v[v.len()/2]
}

fn mode_vec (v: &Vec<i32>) -> Vec<i32> {
    // 出现最多次数
    let mut _mode: Vec<i32> = Vec::new();
    let mut map = HashMap::new();
    for _v in v {
        let count = map.entry(*_v).or_insert(0);  // returns a mutable reference (&mut V)
        *count += 1;
    }
    let mut _max: u8 = 0;
    for (_, v) in &map {
        if *v > _max {
            _max = *v;
        }
    }
    println!("Occure most times: {}", _max);
    for (k, v) in &map {
        if *v == _max {
            _mode.push(*k);
        }
    }
    _mode
}

fn to_pig_latin (s: &String) -> String {
    let mut s1 = s.clone();
    let first_char = s.chars().next().unwrap();
    let vowel = ['a', 'e', 'i', 'o', 'u'];
    if vowel.contains(&first_char) {
        s1.push_str("-hay");
        return s1;
    } else {
        let mut s2 = String::from(&s1[1..]);
        s2.push('-');
        s2.push(first_char);
        return s2 + "ay";
    }
}

fn add_employee (name: String, department: &String, map: &mut HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
    match map.get(department) {
        Some(vec) => vec.push(name),
        None => println!("No such department."),
    }
    // map
}

