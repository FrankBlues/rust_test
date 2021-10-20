use std::process;

extern crate clap;
use clap::{App, Arg};
use gdal::raster::ResampleAlg;
use log::{debug, info, warn};

use gdal_test::{get_files, merge};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("mosaic")
        .version("0.1.0")
        .author("menglimeng")
        .about(
            "Copy valid pixels from input files to an output file, 
        Input files are merged in their listed order.",
        )
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
                .help(
                    "File extensions, if input is a dir, all files with this suffix will be merged",
                )
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

    let config = Params::new(&matches)?;
    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
    Ok(())

    // let output_file = "/data/mosaic113.tif";
    // let output_count: Option<isize> = None;
    // let output_bounds: Option<[f64; 4]> = None;
    // let output_res: Option<(f64, f64)> = None;
    // // let output_res: Option<(f64, f64)> = Some((3e-5, 4e-5));
    // let output_nodata: Option<f64> = None;
    // let output_nodata: Option<f64> = Some(0.0);
    // let output_band_index: Option<Vec<isize>> = None;
    // let resample_method = ResampleAlg::Bilinear;

    // let mut files = vec![];
    // for entry in get_files("/data/test_mosaic", ".tif").unwrap() {
    //     match entry {
    //         Ok(p) => files.push(p.to_str().unwrap().to_owned()),
    //         Err(e) => println!("{:?}", e),
    //     }
    // }
    // merge(files, output_file, output_res, output_nodata, output_count, output_bounds, output_band_index, resample_method);
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
    output_band_index: Option<Vec<isize>>,
}

impl<'a> Params<'a> {
    pub fn new(matches: &'a clap::ArgMatches) -> Result<Params<'a>, &'static str> {
        let mut files = vec![];
        if let Some(param) = matches.value_of("in_rasters") {
            if Path::new(param).is_dir() {
                // Parse extension params
                let mut formats: Vec<&'a str> = Vec::new();
                if let Some(ext) = matches.value_of("extensions") {
                    formats = ext.trim().split_whitespace().collect();
                }
                info!(
                    "Input is a directory, mosaic all files with extension({:?}) in it.",
                    formats
                );
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
        debug!("Parsed input files: {:?}", files);

        let mut out_file: &'a str = "";
        if let Some(param) = matches.value_of("out_raster") {
            out_file = param;
        }

        let mut resample_method = ResampleAlg::Bilinear;
        if let Some(param) = matches.value_of("resample_method") {
            match param {
                "Average" => resample_method = ResampleAlg::Average,
                "Bilinear" => resample_method = ResampleAlg::Bilinear,
                "Cubic" => resample_method = ResampleAlg::Cubic,
                "NearestNeighbour" => resample_method = ResampleAlg::NearestNeighbour,
                _ => warn!("cannot parse resample_method from input, use Bilinear method"),
            }
        }

        let mut output_count: Option<isize> = None;
        if let Some(param) = matches.value_of("output_count") {
            if param.trim().to_lowercase() == "none" {
                output_count = None;
            } else {
                output_count = Some(param.trim().parse::<isize>().unwrap());
            }
        }

        let mut output_bounds: Option<[f64; 4]> = None;
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

        let mut output_res: Option<(f64, f64)> = None;
        if let Some(param) = matches.value_of("output_res") {
            if param.trim().to_lowercase() == "none" {
                output_res = None;
            } else {
                let mut res: Vec<f64> = Vec::new();
                for r in param.trim().split_whitespace() {
                    res.push(r.parse::<f64>().unwrap());
                }
                if res.len() == 1 {
                    output_res = Some((res[0], res[0]));
                } else if res.len() == 2 {
                    output_res = Some((res[0], res[1]));
                } else {
                    warn!("Input res param not equal 1 or 2, try to use the first two");
                    output_res = Some((res[0], res[1]));
                }
            }
        }

        let mut output_nodata: Option<f64> = None;
        if let Some(param) = matches.value_of("output_nodata") {
            if param.trim().to_lowercase() == "none" {
                output_nodata = None;
            } else {
                output_nodata = Some(param.trim().parse::<f64>().unwrap());
            }
        }

        let mut output_band_index: Option<Vec<isize>> = None;
        if let Some(param) = matches.value_of("output_band_index") {
            if param.trim().to_lowercase() == "none" {
                output_band_index = None;
            } else {
                let mut ids: Vec<isize> = Vec::new();
                for b in param.trim().split_whitespace() {
                    ids.push(b.parse::<isize>().unwrap());
                }
                output_band_index = Some(ids);
            }
            debug!("Parsed band_index: {:?}", output_band_index);
        }

        Ok(Params {
            in_rasters: files,
            out_file: out_file,
            resample_method: resample_method,
            output_count: output_count,
            output_bounds: output_bounds,
            output_res: output_res,
            output_nodata: output_nodata,
            output_band_index: output_band_index,
        })
    }
}

/// main program.
fn run(args: Params) -> Result<(), Box<dyn std::error::Error>> {
    merge(
        args.in_rasters,
        args.out_file,
        args.output_res,
        args.output_nodata,
        args.output_count,
        args.output_bounds,
        args.output_band_index,
        args.resample_method,
    )?;
    Ok(())
}
