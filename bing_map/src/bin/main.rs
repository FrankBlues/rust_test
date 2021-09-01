use std::time::SystemTime;
use std::path::{Path, PathBuf};

use bing_map::TilesExtent;
use bing_map::download_util::download_files;
use bing_map::download_util::constuct_url;
use bing_map::download_util::download_files_async;
// use bing_map::download_util::download_files_async1;

use tokio;

#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
// fn main()-> Result<(), reqwest::Error>{
    let (lon0, lat0) =(116.177641, 39.924175);
    let (lon1, lat1) =(116.183095, 39.921244);
    let level = 18;
    let tile_dir = Path::new("D:/temp11").join((&level).to_string());
    if !tile_dir.exists() {
        println!("Dir not exists, create it!");
        std::fs::create_dir_all(&tile_dir).unwrap();
    }
    let single_thread = false;
    let te = TilesExtent::new(lon0, lat0, lon1, lat1, level);
    let tiles = te.tiles();
    for (x, y) in tiles.iter() {
        println!("{},{}", x, y);
    }
    let quad_keys = te.quad_keys();

    let mut urls_files: Vec<(String, PathBuf)> = Vec::with_capacity(quad_keys.len());
    for q in &quad_keys {
        let url = constuct_url(q, "a");
        let path = tile_dir.join(format!("{}{}", q, ".jpeg"));
        urls_files.push((url, path));
    }

    // if single_thread {
    //     // 顺序下载
    //     let st_time = SystemTime::now();
    //     download_files(urls_files);
    //     let lt_time = SystemTime::now();
    //     println!("{:?}", SystemTime::duration_since(&lt_time, st_time).unwrap());
    // } else {
    //     // concurrent
    //     let st_time = SystemTime::now();
    //     download_files_async(urls_files).await;
    //     let lt_time = SystemTime::now();
    //     println!("{:?}", SystemTime::duration_since(&lt_time, st_time).unwrap());
    // }

    Ok(())
}