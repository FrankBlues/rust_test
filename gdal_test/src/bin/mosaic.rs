use std::process;

extern crate clap;
use clap::{App, Arg};
use gdal::raster::ResampleAlg;

use gdal_test::{get_files, merge};

fn main() {

    let matches = App::new("mosaic")
    .version("0.1.0")
    .author("menglimeng")
    .about("Copy valid pixels from input files to an output file, 
        Input files are merged in their listed order.")
    .arg(
        Arg::with_name("in_rasters")
            .long("input")
            .value_name("/mnt/1.tif;/mnt/2.tif")
            .help("Input images, a dir or files seperated by ';'.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("out_raster")
            .long("output")
            .value_name("/mnt/mosaic.tif")
            .help("Output image.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("extensions")
            .long("extension")
            .value_name(".tif")
            .help("File extensions, if input is a dir, all files with this suffix will be merged")
            .default_value(".tif .img")
            .takes_value(true)
            .required(false),
    )
    .arg(
        Arg::with_name("resample_method")
            .long("resample_method")
            .possible_values(&["Average", "Bilinear", "Cubic", "NearestNeighbour"])
            .default_value("Bilinear")
            .help("Resample method.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("output_count")
            .long("output_count")
            .value_name("3")
            .default_value("None")
            .help("The output band count, an integer or None.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("output_bounds")
            .long("output_bounds")
            .value_name("1.0 2.0 3.0 4.0")
            .default_value("None")
            .help("Output boundary [left, bottom, right, top] or None.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("output_res")
            .long("output_res")
            .value_name("1.0 2.0")
            .default_value("None")
            .help("Output resolution (x_res, y_res) or None.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("output_nodata")
            .long("output_nodata")
            .value_name("0.0")
            .default_value("None")
            .help("Output output_nodata (x_res, y_res) or None.")
            .takes_value(true)
            .required(true),
    )
    .arg(
        Arg::with_name("output_band_index")
            .long("output_band_index")
            .value_name("1 2 3")
            .default_value("None")
            .help("Output band index, vec or None.")
            .takes_value(true)
            .required(true),
    )
    .get_matches();



    env_logger::init();

    let output_file = "/data/mosaic113.tif";
    let output_count: Option<isize> = None;
    let output_bounds: Option<[f64; 4]> = None;
    let output_res: Option<(f64, f64)> = None;
    // let output_res: Option<(f64, f64)> = Some((3e-5, 4e-5));
    let output_nodata: Option<f64> = None;
    let output_nodata: Option<f64> = Some(0.0);
    let output_band_index: Option<Vec<isize>> = None;
    let resample_method = ResampleAlg::Bilinear;

    let mut files = vec![];
    for entry in get_files("/data/test_mosaic", ".tif").unwrap() {
        match entry {
            Ok(p) => files.push(p.to_str().unwrap().to_owned()),
            Err(e) => println!("{:?}", e),
        }
    }
    merge(files, output_file, output_res, output_nodata, output_count, output_bounds, output_band_index, resample_method);

}


use std::path::Path;

/// Parse input args.
pub struct Params<'a> {
    in_rasters: Vec<String>,
    out_file: &'a str,
    resample_method: ResampleAlg,
    output_count: Option<isize>,
    output_bounds: Option<[f64; 4]>,
    output_res: Option<(f64, f64)>,
    output_nodata: Option<f64>,
    output_band_index: Option<Vec<isize>>
}

impl<'a> Params<'a> {
    pub fn new(matches: &'a clap::ArgMatches) -> Result<Params<'a>, &'static str> {
        let mut files = vec![];
        if let Some(param) = matches.value_of("in_rasters") {
            if Path::new(param).is_dir(){
                // Parse extension params
                let mut formats: Vec<&'a str> = Vec::new();
                if let Some(ext) = matches.value_of("extensions") {
                    formats = ext.trim().split_whitespace().collect();
                }
                for f in formats {
                    for entry in get_files(param, f).unwrap() {
                        match entry {
                            Ok(p) => {
                                let file_name = p.to_str().unwrap().to_owned();
                                if !Path::new(&file_name).is_file() {
                                    println!("Not a file, {}", file_name);
                                    continue;
                                }
                                files.push(file_name);
                            }
                            Err(e) => println!("{:?}", e),
                        }
                    }
                }
            } else {
                for f in param.trim().split(";") {
                    let file_name = String::from(f);
                    if !Path::new(&file_name).is_file() {
                        println!("Not a file, {}", file_name);
                        continue;
                    }
                    files.push(file_name);
                }
            }
        }

        let mut out_file: &'a str = "";
        if let Some(param) = matches.value_of("out_raster") {
            out_file = param;
        }

        let resample_method;
        if let Some(param) = matches.value_of("resample_method") {
            match param {
                "Average" => resample_method = ResampleAlg::Average,
                "Bilinear" => resample_method = ResampleAlg::Bilinear,
                "Cubic" => resample_method = ResampleAlg::Cubic,
                "NearestNeighbour" => resample_method = ResampleAlg::NearestNeighbour,
                _ => ()
            }
        }

        let output_count: Option<isize>;
        if let Some(param) = matches.value_of("output_count") {
            if param.trim().to_lowercase() == "none" {
                output_count = None;
            }else{
                output_count = Some(param.trim().parse::<isize>().unwrap());
            }
            
        }

        let output_bounds: Option<[f64; 4]>;
        if let Some(param) = matches.value_of("output_bounds") {
            if param.trim().to_lowercase() == "none" {
                output_bounds = None;
            } else {
                let mut bouds = [0.0; 4];
                for (i, b) in param.trim().split_whitespace().enumerate() {
                    bouds[i] = b.parse::<f64>().unwrap();
                }
                output_bounds = Some(bouds);
            }
        }

        let output_res: Option<(f64, f64)>;
        if let Some(param) = matches.value_of("output_res") {
            if param.trim().to_lowercase() == "none" {
                output_res = None;
            } else {
                output_res = Some(param.trim().split_whitespace().collect::(f64, f64)());
            }
        }

        let output_nodata: Option<f64>;
        if let Some(param) = matches.value_of("output_nodata") {
            if param.trim().to_lowercase() == "none" {
                output_nodata = None;
            }else{
                output_nodata = Some(param.trim().parse::<f64>().unwrap());
            }
        }

        let output_band_index: Option<Vec<isize>>;
        if let Some(param) = matches.value_of("output_band_index") {
            if param.trim().to_lowercase() == "none" {
                output_band_index = None;
            } else {
                let mut ids: Vec<isize> = Vec::new();
                for b in param.trim().split_whitespace() {
                    ids.push(b.parse::<f64>().unwrap());
                }
                output_band_index = Some(ids);
            }
        }


        Ok(Params {
            in_rasters: files,
            out_file: out_file,
            image_dir: image_dir,
            resample_method: resample_method,
            output_count: output_count,
            output_bounds: output_bounds,
            output_res: output_res,
            output_nodata: output_nodata,
            output_band_index: output_band_index
        })
    }
}

/// main program.
fn run(args: Params) -> Result<(), Box<dyn std::error::Error>> {
    
}