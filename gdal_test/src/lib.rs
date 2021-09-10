use gdal::raster::{Buffer, GdalType, RasterBand};
use gdal::{Dataset, Metadata, Driver};
use gdal_sys::GDALDataType;
use std::path::Path;

// #[cfg(feature = "ndarray")]
use ndarray::Array2;
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

    #[test]
    fn test_gdal_raster_read_write() {
        let in_raster = "/data/read_write_raster/road.tif";
        let out_raster = "/data/trans_raster.tif";

        let val_value: u8 = 158;
        let (r, g, b) = (22 as u8, 181 as u8, 255 as u8);
        let ds = Dataset::open(Path::new(in_raster)).unwrap();
        let count = ds.raster_count();
        if count != 1 {
            println!("Warning: input raster has more than 1 ({}) band", &count);
            println!("Try using the first band.");
        }
        let rb = ds.rasterband(1).unwrap();
        let (x_size, y_size) = ds.raster_size();
        let dtype = rb.band_type();
        println!("{:?}", rb.block_size());
        println!("{:?}", rb.actual_block_size(((x_size/128) as isize, 0)));
        println!("{:?}", rb.actual_block_size((0, (y_size/128) as isize)));
        println!("{:?}", rb.actual_block_size(((x_size/128) as isize, (y_size/128) as isize)));
        // type T = u8;
        // match dtype {
        //     1 => {type T = u8},
        //     _ => (),
        // }

        type T= u8;

        let mut rb_data: Buffer<T> = rb.read_band_as().unwrap();
        
        // ndarray
        // let mut arr = Array2::from_shape_vec(
        //     (y_size, x_size),
        //     rb_data.data,
        // ).unwrap();
        
        // let mut arr1 = arr.clone();
        // let mut arr2 = arr.clone();
        // arr[arr == 158] = 22;


        // Output
        {
            let driver = Driver::get("GTiff").unwrap();
            let mut new_ds = driver
                .create_with_band_type::<T>(
                    out_raster,
                    rb.x_size() as isize,
                    rb.y_size() as isize,
                    3,
                )
                .unwrap();
            new_ds
                .set_geo_transform(&ds.geo_transform().unwrap())
                .unwrap();
            new_ds.set_spatial_ref(&ds.spatial_ref().unwrap()).unwrap();
            // band 1
            
            // unsafe pointer
            // let x_ptr = &rb_data.data.as_mut_ptr();
            // unsafe {
            //     for i in 0..(x_size * y_size) {
            //         if *x_ptr.add(i) == val_value {
            //             *x_ptr.add(i) = r;
            //         }
            //     }
            // }

            // for loop
            // for d in &mut rb_data.data {
            //     if *d == val_value {
            //         *d = r;
            //     }
            // }

            // ndarray
            // arr.mapv_inplace(|x| if x == val_value {r} else {x});
            // rb_data.data = arr.into_raw_vec();

            // for each
            &rb_data.data.iter_mut().for_each(|x| {
                if *x == val_value {
                    *x = r;
                }
            });
            // rb_data.data = bd;
            let mut new_rb = new_ds.rasterband(1).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb
                .write((0, 0), (x_size, y_size), &rb_data)
                .unwrap();
            // band 2
            // unsafe pointer
            // unsafe {
            //     for i in 0..(x_size * y_size) {
            //         if *x_ptr.add(i) == r {
            //             *x_ptr.add(i) = g;
            //         }
            //     }
            // }

            // iter.map(|mut x| { if x == r {
            //     x = g;
            // }});
            
            // arr.mapv_inplace(|x| if x == val_value {r} else {x});
            // for d in &mut rb_data.data {
            //     if *d == r {
            //         *d = g;
            //     }
            // }
            
            // ndarray
            // arr1.mapv_inplace(|x| if x == val_value {g} else {x});
            // rb_data.data = arr1.into_raw_vec();

            // for each
            &rb_data.data.iter_mut().for_each(|x| {
                if *x == r {
                    *x = g;
                }
            });
            let mut new_rb = new_ds.rasterband(2).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb
                .write((0, 0), (x_size, y_size), &rb_data)
                .unwrap();

            // band 3
            // unsafe pointer
            // unsafe {
            //     for i in 0..(x_size * y_size) {
            //         if *x_ptr.add(i) == g {
            //             *x_ptr.add(i) = b;
            //         }
            //     }
            // }

            // iter.map(|mut x| { if x == g {
            //     x = b;
            // }});

            // for d in &mut rb_data.data {
            //     if *d == g {
            //         *d = b;
            //     }
            // }

            // ndarray
            // arr2.mapv_inplace(|x| if x == val_value {b} else {x});
            // rb_data.data = arr2.into_raw_vec();

            // for each
            &rb_data.data.iter_mut().for_each(|x| {
                if *x == g {
                    *x = b;
                }
            });
            let mut new_rb = new_ds.rasterband(3).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb
                .write((0, 0), (x_size, y_size), &rb_data)
                .unwrap();
        }
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
