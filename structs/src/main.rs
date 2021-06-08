fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1'name: {}.", user1.username);

    // 可改变
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");
    println!("user1'email: {}.", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1  // 其余与user1相同
    };
    println!("user2'name: {}.", user2.username);

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);  // tuple structs
    let origin = Point(0, 0, 0);

}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn build_user(email: String, username: String) -> User {
    User {
        email,  // 与结构体中有同名参数,不用加email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}