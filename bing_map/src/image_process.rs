extern crate image;
use image::ImageBuffer;
use std::path::Path;

use threadpool::ThreadPool;

use std::sync::{Arc, Mutex};
use std::thread;

/// Merge all map tiles within a extent cocurrently use thread pool
pub fn merge_tiles<T: AsRef<Path>>(
    tile0: (usize, usize),
    tile1: (usize, usize),
    merged_file: T,
    tile_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let pool = ThreadPool::new(8);
    let (tile_x_s, tile_y_s) = (tile0.0 as u32, tile0.1 as u32);
    let (tile_x_e, tile_y_e) = (tile1.0 as u32, tile1.1 as u32);
    let merged: image::RgbImage = ImageBuffer::new(
        (tile_x_e - tile_x_s + 1) * 256,
        (tile_y_e - tile_y_s + 1) * 256,
    );
    let merged1 = Arc::new(Mutex::new(merged));
    for tile_x in tile_x_s..=tile_x_e {
        for tile_y in tile_y_s..=tile_y_e {
            let img_path = tile_dir
                .join(tile_x.to_string())
                .join(tile_y.to_string() + ".jpeg");
            if !img_path.is_file() {
                println!("Warning: file ({}) not exist", img_path.to_str().unwrap());
                continue;
            }
            match image::open(&img_path) {
                Ok(img_obj) => {
                    let img = img_obj.to_rgb8();
                    let merged1 = merged1.clone();
                    pool.execute(move || {
                        write_one_tile1(merged1, tile_x, tile_y, tile_x_s, tile_y_s, img);
                    })
                }
                Err(e) => {
                    println!("Warning: open image ({}) fail.", img_path.to_str().unwrap());
                    eprintln!("Error: {}", e);
                    continue;
                }
            }
            // let img = image::open(&img_path).unwrap().to_rgb8();
        }
    }
    pool.join();
    (*merged1).lock().unwrap().save(merged_file).unwrap();
    Ok(())
}

/// Merge all map tiles within a extent cocurrently use thread::spawn.
pub fn merge_tiles2<T: AsRef<Path>>(
    tile0: (usize, usize),
    tile1: (usize, usize),
    merged_file: T,
    tile_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];
    let (tile_x_s, tile_y_s) = (tile0.0 as u32, tile0.1 as u32);
    let (tile_x_e, tile_y_e) = (tile1.0 as u32, tile1.1 as u32);
    let merged: image::RgbImage = ImageBuffer::new(
        (tile_x_e - tile_x_s + 1) * 256,
        (tile_y_e - tile_y_s + 1) * 256,
    );
    let merged1 = Arc::new(Mutex::new(merged));
    for tile_x in tile_x_s..=tile_x_e {
        for tile_y in tile_y_s..=tile_y_e {
            let img_path = tile_dir
                .join(tile_x.to_string())
                .join(tile_y.to_string() + ".jpeg");
            if !img_path.is_file() {
                println!("Warning: file ({}) not exist", img_path.to_str().unwrap());
                continue;
            }

            match image::open(&img_path) {
                Ok(img_obj) => {
                    let img = img_obj.to_rgb8();
                    let merged1 = merged1.clone();
                    let handle = thread::spawn(move || {
                        write_one_tile1(merged1, tile_x, tile_y, tile_x_s, tile_y_s, img)
                    });
                    handles.push(handle);
                }
                Err(e) => {
                    println!("Warning: open image ({}) fail.", img_path.to_str().unwrap());
                    eprintln!("Error: {}", e);
                    continue;
                }
            }
        }
    }
    for handle in handles {
        handle.join().unwrap();
    }

    (*merged1).lock().unwrap().save(merged_file).unwrap();
    Ok(())
}

/// Merge all map tiles within a extent use single thread.
pub fn merge_tiles1<T: AsRef<Path>>(
    tile0: (usize, usize),
    tile1: (usize, usize),
    merged_file: T,
    tile_dir: &std::path::Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let (tile_x_s, tile_y_s) = (tile0.0 as u32, tile0.1 as u32);
    let (tile_x_e, tile_y_e) = (tile1.0 as u32, tile1.1 as u32);
    let mut merged: image::RgbImage = ImageBuffer::new(
        (tile_x_e - tile_x_s + 1) * 256,
        (tile_y_e - tile_y_s + 1) * 256,
    );
    for tile_x in tile_x_s..=tile_x_e {
        for tile_y in tile_y_s..=tile_y_e {
            let img_path = tile_dir
                .join(tile_x.to_string())
                .join(tile_y.to_string() + ".jpeg");
            if !img_path.is_file() {
                println!("Warning: file ({}) not exist", img_path.to_str().unwrap());
                continue;
            }

            match image::open(&img_path) {
                Ok(img_obj) => {
                    let img = img_obj.to_rgb8();
                    write_one_tile(&mut merged, tile_x, tile_y, tile_x_s, tile_y_s, img);
                }
                Err(e) => {
                    println!("Warning: open image ({}) fail.", img_path.to_str().unwrap());
                    eprintln!("Error: {}", e);
                    continue;
                }
            }
        }
    }
    merged.save(merged_file).unwrap();
    Ok(())
}

/// Write one tile to a image::ImageBuffer object.
pub fn write_one_tile(
    merged: &mut image::RgbImage,
    tile_x: u32,
    tile_y: u32,
    tile_x_s: u32,
    tile_y_s: u32,
    img: image::RgbImage,
) {
    for (x, y, pixel) in img.enumerate_pixels() {
        // 计算对应合并后的位置
        // let dst_pixel = merged.get_pixel_mut((tile_x-start_x) * 256 + x, (tile_y - start_y) * 256 + y);
        // *dst_pixel = *pixel;
        merged.put_pixel(
            (tile_x - tile_x_s) * 256 + x,
            (tile_y - tile_y_s) * 256 + y,
            *pixel,
        );
    }
}

/// Write one tile to a packed image::ImageBuffer object to allow transfer between threads.
pub fn write_one_tile1(
    merged: Arc<Mutex<image::RgbImage>>,
    tile_x: u32,
    tile_y: u32,
    tile_x_s: u32,
    tile_y_s: u32,
    img: image::RgbImage,
) {
    // let img = image::open(&img_path).unwrap().to_rgb8();
    let mut mu_merged = merged.lock().unwrap();
    for (x, y, pixel) in img.enumerate_pixels() {
        (*mu_merged).put_pixel(
            (tile_x - tile_x_s) * 256 + x,
            (tile_y - tile_y_s) * 256 + y,
            *pixel,
        );
    }
}
