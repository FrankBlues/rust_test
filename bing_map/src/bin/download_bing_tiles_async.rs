use bing_map::{run, Config};
use std::process;

use tokio;

extern crate clap;
use clap::{App, Arg};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("bing map")
        .version("0.1.0")
        .author("menglimeng")
        .about("Download bing map tile within the designated extent and merged all the tiles to one image(.png) with the world file(.pnw).")
        .arg(
            Arg::with_name("ul_lonlat")
                .short("u")
                .long("ul_lonlat")
                .value_name("116.177641 39.924175")
                .default_value("116.177641 39.924175")
                .help("Coordinats of the upper left corner.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("br_lonlat")
                .short("b")
                .long("br_lonlat")
                .value_name("116.183095 39.921244")
                .default_value("116.183095 39.921244")
                .help("Coordinats of the bottom right corner.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("zoom_level")
                .short("z")
                .long("zoom")
                .value_name("18")
                .default_value("15")
                .help("The zoom level.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("tiles_dir")
                .short("d")
                .long("tiles_dir")
                .default_value(".")
                .help("The downloaded tiles directory.")
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("./out.png")
                .help("The output merged image.")
                .takes_value(true)
                .required(true)
        )
        .get_matches();

    let config = Config::new(matches)?;
    if let Err(e) = run(config).await {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }

    Ok(())
}
