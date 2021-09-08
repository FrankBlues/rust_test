use std::path::Path;
use std::time::SystemTime;

use bing_map::download_util::download_files;
use bing_map::merge_tiles;
use bing_map::write_string_to_text;
use bing_map::TilesExtent;

fn main() {
    let (lon0, lat0) = (116.1, 39.95);
    let (lon1, lat1) = (116.15, 39.9);
    let level = 18;
    let tile_dir = Path::new("D:/temp11").join((&level).to_string());
    let out_png = "d:/merged_test.png";

    let world_file = out_png.replace(".png", ".pgw");

    let te = TilesExtent::new(lon0, lat0, lon1, lat1, level);
    let urls_files = te.construct_download_params(&tile_dir);
    // for (u, f) in urls_files.iter() {
    //     let path = f.to_str().unwrap();
    //     if path.contains("215617") {
    //         println!("{}: {:?}", u, path);
    //     }
    // }
    let (tile0, tile1) = te.tile_extent();
    let world_file_content = te.gen_world_file_content(&tile0);

    // download one by one
    println!("Download start!");
    let st_time = SystemTime::now();
    download_files(&urls_files);
    let lt_time = SystemTime::now();
    println!(
        "{} tiles downloaded, spend {:?}",
        &urls_files.len(),
        SystemTime::duration_since(&lt_time, st_time).unwrap()
    );

    println!("Merging the tiles.");
    merge_tiles(tile0, tile1, out_png, &tile_dir).unwrap();

    println!("Generate world file.");
    write_string_to_text(&world_file, world_file_content).unwrap();
}
