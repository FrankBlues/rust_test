
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

    let v = vec![3, 5, 10, 56, 78, 3, 20, 15, 30, 5, 78, 10, 12];
    // let mut sum = 0;
    // for i in &v {
    //     sum += *i;
    // }
    println!("Mean of v: {}", mean_vec(&v));
    println!("{}", v[2]);

}

fn mean_vec (v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v {
        sum += *i;
    }
    sum
}
