use std::fs::read;
use std::path::Path;

fn main() {
    let pic = "d:/temp11/bing_18/18/215617/99327.jpeg";
    let url = "http://ecn.t5.tiles.virtualearth.net/tiles/a132100103223222223.jpeg?g=129";
    let res = reqwest::blocking::get(url).unwrap();
    println!("{:?}", res.bytes().unwrap().len());
    println!("{:?}", read(pic).unwrap().len());
    println!("Hello, world!");
}
