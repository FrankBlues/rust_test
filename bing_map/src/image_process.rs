extern crate image;
use image::ImageBuffer;

/// Merge all map tiles within a square extent
/// 修改成并行
///
pub fn merge_tiles(
    tile0: (usize, usize),
    tile1: (usize, usize),
    merged_file: &str,
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
                println!(
                    "Warning: file ({}) not exist, pass to the next.",
                    img_path.to_str().unwrap()
                );
                continue;
            }
            let img = image::open(&img_path).unwrap().to_rgb8();
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
    }

    merged.save(merged_file).unwrap();
    Ok(())
}
