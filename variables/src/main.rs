fn main() {
    let x = 5;  // 不能直接改变x的值,可以用let重新赋值
    println!("The value of x is: {}", x);
    let mut x = 6;  // 可以改变x的值 但是同类型
    x = 7;
    println!("The value of x is: {}", x);

    const MAX_POINTS: u32 = 100_000;
    println!("The MAX_POINTS is: {}", MAX_POINTS);

    let x = x * 2;  // 可以多次改变x的值
    println!("The value of x is: {}", x);

    let spaces = "   ";  // 甚至可以改变类型
    let spaces = spaces.len();
}
