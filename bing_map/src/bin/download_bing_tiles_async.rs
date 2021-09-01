use std::time::SystemTime;

use bing_map::TilesExtent;
use bing_map::download_util::download_files_async;
// use bing_map::download_util::download_files_async1;

use tokio;

#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    let (lon0, lat0) =(116.177641, 39.924175);
    let (lon1, lat1) =(116.183095, 39.921244);
    let level = 18;
    let tile_dir = std::path::Path::new("D:/temp11").join((&level).to_string());

    let te = TilesExtent::new(lon0, lat0, lon1, lat1, level);
    let urls_files = te.construct_download_params(tile_dir);

    //download concurrently
    let st_time = SystemTime::now();
    download_files_async(urls_files).await;
    let lt_time = SystemTime::now();
    println!("{:?}", SystemTime::duration_since(&lt_time, st_time).unwrap());

    Ok(())
}