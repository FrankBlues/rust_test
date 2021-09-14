extern crate clap;
use clap::{App, Arg};

use gdal_test::{Config, run};
use std::process;


fn main () -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("gray2rgb")
        .version("0.1.0")
        .author("menglimeng")
        .about("Convert gray image to rgb images.")
        .arg(
            Arg::with_name("in_raster")
                .short("i")
                .long("input")
                .value_name("/mnt/gray.tif")
                .help("Input image with only one band.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("out_raster")
                .short("o")
                .long("output")
                .value_name("/mnt/rgb.tif")
                .help("Output image with three band in the RGB order.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("valid")
                .short("v")
                .long("valid_value")
                .value_name("1")
                .default_value("1")
                .help("The valid value in the input image.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("rgb")
                .short("c")
                .long("rgb_value")
                .value_name("255 255 255")
                .default_value("255 255 255")
                .help("The rgb value in the output image.")
                .takes_value(true)
                .required(true)
        )
        .get_matches();
    
    let config = Config::new(matches)?;
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    Ok(())
}


