use std::path::Path;
use std::time::SystemTime;

use bing_map::download_util::download_files_async;
use bing_map::TilesExtent;
// use bing_map::download_util::download_files_async1;
use bing_map::merge_tiles;
use bing_map::write_string_to_text;

use tokio;

extern crate clap;
use clap::{Arg, App};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("bing map")
        .version("0.1.0")
        .author("author")
        .about("Does awesome things")
        .arg(Arg::with_name("ul_lonlat")
            .short("ul")
            .long("ul_lonlat")
            .value_name("")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    if let Some(c) = matches.value_of("ul_lonlat") {
        println!("Value for config: {}", c);
    }
    let (lon0, lat0) = (116.177641, 39.924175);
    let (lon1, lat1) = (116.183095, 39.921244);
    let level = 18;
    let tile_dir = Path::new("D:/temp11").join((&level).to_string());
    let out_png = "d:/merged_test.png";

    let world_file = out_png.replace(".png", ".pgw");

    let te = TilesExtent::new(lon0, lat0, lon1, lat1, level);
    let urls_files = te.construct_download_params(&tile_dir);
    let (tile0, tile1) = te.tile_extent();
    let world_file_content = te.gen_world_file_content(&tile0);

    //download concurrently
    println!("Download start!");
    let st_time = SystemTime::now();
    download_files_async(&urls_files).await;
    let lt_time = SystemTime::now();
    println!(
        "{} tiles downloaded, spend {:?}",
        &urls_files.len(),
        SystemTime::duration_since(&lt_time, st_time).unwrap()
    );

    println!("Merging the tiles.");
    merge_tiles(tile0, tile1, out_png, &tile_dir)?;

    println!("Generate world file.");
    write_string_to_text(&world_file, world_file_content)?;
    Ok(())
}
