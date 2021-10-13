extern crate clap;
use clap::{App, Arg};

use gdal_test::{run_find_intersected, ParamsFindIntersected};
use std::process;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("find_images_intersected.")
        .version("0.1.0")
        .author("menglimeng")
        .about("Find all the images in a drectory that intersecing with an input image, write to a txt file.")
        .arg(
            Arg::with_name("in_raster")
                .short("i")
                .long("input")
                .value_name("/mnt/input.tif")
                .help("Input image.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("out_file")
                .short("o")
                .long("output")
                .value_name("/mnt/result.txt")
                .help("Output txt file.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("image_dir")
                .short("d")
                .long("image_dir")
                .value_name("/data/gf2/")
                .help("Directory contain images.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("formats")
                .short("f")
                .long("formats")
                .value_name(".tif .img")
                .default_value(".tif .img")
                .help("The file suffix, may be more than one.")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let config = ParamsFindIntersected::new(&matches)?;
    if let Err(e) = run_find_intersected(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    Ok(())
}
