use gdal::raster::{Buffer, GdalType, RasterBand};
use gdal::{Dataset, Metadata};
use gdal_sys::GDALDataType;
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_gdal_raster_read() {
        let mut dataset = Dataset::open(Path::new("/data/南屯村2.img")).unwrap();
        // println!("{:?}", dataset.projection());
        // let sr = dataset.spatial_ref().unwrap();
        // println!("{}", sr.is_projected());
        // let driver = dataset.driver();
        // println!("{}, {}", driver.long_name(), driver.short_name());
        // // dataset.build_overviews("AVERAGE", &[2, 4, 8], &[]);
        let bands = dataset.raster_count();
        let (cols, rows) = dataset.raster_size();
        println!("bands:{}, dims:{}, {}", &bands, cols, rows);

        let gt = dataset.geo_transform().unwrap();
        println!("geo transform: {:?}", gt);
        let (ul_x, ul_y) = (gt[0], gt[3]);
        let res = gt[1];

        // // let b1 = dataset.rasterband(1);
        // let driver = gdal::Driver::get("mem").unwrap();
        // println!("driver description: {:?}", driver.description());

        // let path = Path::new("/data/南屯村2.img");
        // let dataset = Dataset::open(path).unwrap();
        // println!("dataset description: {:?}", dataset.description());

        // let key = "INTERLEAVE";
        // let domain = "IMAGE_STRUCTURE";
        // let meta = dataset.metadata_item(key, domain);
        // println!("domain: {:?} key: {:?} -> value: {:?}", domain, key, meta);
        println!("metadata domains: {:?} ", dataset.metadata_domains());

        let rasterband: RasterBand = dataset.rasterband(1).unwrap();
        println!("rasterband description: {:?}", rasterband.description());
        println!("rasterband no_data_value: {:?}", rasterband.no_data_value());
        let band_type = rasterband.band_type();
        println!("rasterband type: {:?}", &band_type);
        println!("rasterband scale: {:?}", rasterband.scale());
        println!("rasterband offset: {:?}", rasterband.offset());

        // if let Ok(rv) = rasterband.read_as::<u8>((20, 30), (2, 3), (2, 3), None) {
        //     println!("{:?}", rv.data);
        // }
        let band_data_buffer: Buffer<u8> = rasterband.read_band_as().unwrap();
        println!("{:?}", band_data_buffer.size);
        let band_data = band_data_buffer.data;
        println!("{}", band_data.len());

        // OverView
        // println!("overview_count: {:?}", rasterband.overview_count());
        // let ov0 = rasterband.overview(3).unwrap();
        // println!("{:?}, {:?}", ov0.size(), ov0.overview_count());

        let (x, y) = (36612298.477, 4056464.768);
        let (pixel_x, pixel_y) = xy2pixel_xy(x, y, &gt);
        println!(
            "pixel at ({}, {}): {}",
            pixel_x,
            pixel_y,
            &band_data[pixel_y * cols + pixel_x]
        );
        println!("xy: {:?}", pixel_xy2xy(pixel_x, pixel_y, &gt));
    }
}

pub fn xy2pixel_xy(x: f64, y: f64, geo_transform: &[f64; 6]) -> (usize, usize) {
    let res = geo_transform[1];
    (
        ((x - geo_transform[0]) / res) as usize,
        ((geo_transform[3] - y) / res) as usize,
    )
}

pub fn pixel_xy2xy(pixel_x: usize, pixel_y: usize, geo_transform: &[f64; 6]) -> (f64, f64) {
    let res = geo_transform[1];
    (
        geo_transform[0] + (pixel_x as f64) * res,
        geo_transform[3] - (pixel_y as f64) * res,
    )
}
