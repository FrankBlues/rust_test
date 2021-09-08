use std::fs::read;
use std::path::Path;
use std::rc::Rc;
use std::sync::Mutex;
use std::cell::RefCell;

fn main() {
    let pic = "d:/temp11/bing_18/18/215617/99327.jpeg";
    let url = "http://ecn.t5.tiles.virtualearth.net/tiles/a132100103223222223.jpeg?g=129";
    let res = Rc::new(RefCell::new(reqwest::blocking::get(url).unwrap()));
    let res1 = res.clone();
    let res_ = res1.borrow();
    // println!("{:?}", (*res_).bytes().unwrap().len());
    println!("{:?}", read(pic).unwrap().len());
    println!("Hello, world!");
}
