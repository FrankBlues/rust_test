use std::time::SystemTime;
use std::path::Path;
use std::path::PathBuf;
use bing_map::TilesExtent;
use bing_map::download_util::download_files_async;
// use bing_map::download_util::download_files_async1;
use bing_map::get_files;

extern crate image;

use image::{GenericImageView, ImageBuffer};

use tokio;

#[tokio::main]
async fn main()-> Result<(), reqwest::Error> {
    let (lon0, lat0) =(116.177641, 39.924175);
    let (lon1, lat1) =(116.183095, 39.921244);
    let level = 18;
    let tile_dir = Path::new("D:/temp11").join((&level).to_string());

    // let te = TilesExtent::new(lon0, lat0, lon1, lat1, level);
    // let urls_files = te.construct_download_params(&tile_dir);

    // //download concurrently
    // let st_time = SystemTime::now();
    // download_files_async(urls_files).await;
    // let lt_time = SystemTime::now();
    // println!("{:?}", SystemTime::duration_since(&lt_time, st_time).unwrap());

    let mut merge_files: Vec<String> = Vec::new();
    let files = get_files(&tile_dir).unwrap();

    let (start_x, start_y) = (215669, 99314);
    let mut merged: image::RgbImage = ImageBuffer::new(5 * 256, 4 * 256);

    for tile_x in start_x..=215673 {
        for tile_y in start_y..=99317 {
            let img_path = tile_dir.join(tile_x.to_string()).join(tile_y.to_string() + ".jpeg");
            let mut img = image::open(&img_path).unwrap().to_rgb8();
            for (x, y, pixel) in img.enumerate_pixels_mut() {
                // 计算对应合并后的位置
                let dst_pixel = merged.get_pixel_mut((tile_x-start_x) * 256 + x, (tile_y - start_y) * y);
                *dst_pixel = *pixel;
            }
        }
    }

    merged.save("d:/merged3.png").unwrap();

    // let mut path_flag = tile_dir.join(start_x.to_string()).join(start_y.to_string() + ".jpeg");
    // for (x, y, pixel) in merged.enumerate_pixels_mut() {
    //     let tile_x = start_x + x / 256;
    //     let tile_y = start_y + y / 256;
    //     let img_path = tile_dir.join(tile_x.to_string()).join(tile_y.to_string() + ".jpeg");
        
    //     let mut img = image::open(&path_flag).unwrap().to_rgb8();
    //     if path_flag != img_path {
    //         println!("处理{:?}", path_flag);
    //         path_flag = img_path;
    //         img = image::open(&path_flag).unwrap().to_rgb8();
    //     }
    //     *pixel = *img.get_pixel(x % 256, y % 256);
    // }
    // merged.save("d:/merged2.png").unwrap();


    // for (k, v) in &files {
    //     println!("Merging directory: {:?}", k);
    //     let img = image::open(&v[0]).unwrap();

    //     let mut merged: image::RgbImage = ImageBuffer::new(img.width(), img.height()*(v.len() as u32));

    //     let mut all_bytes: Vec<u8> = img.to_bytes();
    //     for f in &v[1..] {
    //         let mut bytes = image::open(f).unwrap().to_bytes();
    //         all_bytes.append(&mut bytes);
    //     }
    //     merged.copy_from_slice(&mut all_bytes);
    //     let out_file = format!("{}{}", k.to_str().unwrap(), ".png");
    //     merged.save(&out_file).unwrap();
    //     merge_files.push(out_file);
    // }

    // let mut merged: image::RgbImage = ImageBuffer::new(5 * 256, 4 * 256);
    // let mut all_bytes: Vec<u8> = Vec::new();
    // for f in &merge_files {
    //     println!("{}", f);
    //     let mut bytes = image::open(f).unwrap().to_bytes();
    //     all_bytes.append(&mut bytes);
    // }
    // merged.copy_from_slice(&mut all_bytes);
    // merged.save("d:/merged1.png").unwrap();

    Ok(())
}

