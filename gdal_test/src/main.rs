use std::path::Path;
use gdal::Dataset;


fn main() {
    let dataset = Dataset::open(Path::new("D:\temp11\南屯村2.img"));
    println!("{:?}", dataset);
}
