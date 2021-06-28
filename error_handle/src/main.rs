use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;
use std::fs;


fn main() {
    let v = vec!{1, 2, 3};
    // println!("{}", v[99]);
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", error)
            }
        },
    };

    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //  If the Result is the Err variant, unwrap will call the panic! macro for us
    // let f = File::open("hello1.txt").unwrap();

    // in its call to panic! will be the parameter that we pass to expect
    // let f = File::open("hello1.txt").expect("Failed to open hello.txt");

    println!("{:?}", read_username_from_file());
}


fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 上面函数的简化版 ?功能与match基本相同
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// 再简化
fn read_username_from_file_shorter_shoerter() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

// shortest 利用系统提供函数
fn read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}