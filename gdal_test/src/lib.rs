use gdal::raster::{Buffer, GdalType, RasterBand};
use gdal::{Dataset, Driver};
use std::path::Path;
use ndarray::{s, Array2, Zip, ArcArray, Dim};

use threadpool::ThreadPool;
use std::sync::{Mutex, Arc};

// #[cfg(feature = "ndarray")]
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
        println!("{:?}", rb.actual_block_size(((x_size / 128) as isize, 0)));
        println!("{:?}", rb.actual_block_size((0, (y_size / 128) as isize)));
        println!(
            "{:?}",
            rb.actual_block_size(((x_size / 128) as isize, (y_size / 128) as isize))
        );
        // type T = u8;
        // match dtype {
        //     1 => {type T = u8},
        //     _ => (),
        // }

        type T = u8;

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
            new_rb.write((0, 0), (x_size, y_size), &rb_data).unwrap();
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
            new_rb.write((0, 0), (x_size, y_size), &rb_data).unwrap();

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
            new_rb.write((0, 0), (x_size, y_size), &rb_data).unwrap();
        }
    }

    #[test]
    fn test_blocks_ndarray() {
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
        println!("{:?}, {:?}", x_size, y_size);
        let dtype = rb.band_type();
        let (block_x, block_y) = rb.block_size();
        println!("{:?}", rb.block_size());
        let (x_remain, _) = rb.actual_block_size(((x_size / 128) as isize, 0)).unwrap();
        let (_, y_remain) = rb.actual_block_size((0, (y_size / 128) as isize)).unwrap();
        // type T = u8;
        // match dtype {
        //     1 => {type T = u8},
        //     _ => (),
        // }

        // gray2rgb::<u8>(rb, &out_raster, &ds, val_value, r, g, b).unwrap();

        
        type T = u8;

        let mut array = Array2::<u8>::zeros((y_size, x_size));
        let mut array_g = array.clone();
        let mut array_b = array.clone();

        for x in 0..=x_size / block_x {
            for y in 0..=y_size / block_y {
                // println!("Reading {}, {} block", x, y);
                let mut block_arr: Array2<u8> = rb.read_block((x, y)).unwrap();
                // block_arr.for_each(|a| {println!("{}", *a)});
                // println!("{}", block_arr.sum());

                let end_x = if block_x * (x + 1) >= x_size {
                    x_size
                } else {
                    x * 128 + 128
                };
                let end_y = if block_y * (y + 1) >= y_size {
                    y_size
                } else {
                    y * 128 + 128
                };
                if block_x * (x + 1) >= x_size && !block_y * (y + 1) >= y_size {
                    block_arr = block_arr.slice(s!(0..128, 0..x_remain)).to_owned();
                }
                if block_x * (x + 1) < x_size && block_y * (y + 1) >= y_size {
                    block_arr = block_arr.slice(s!(0..y_remain, 0..128)).to_owned();
                }
                if block_x * (x + 1) >= x_size && block_y * (y + 1) >= y_size {
                    block_arr = block_arr.slice(s!(0..y_remain, 0..x_remain)).to_owned();
                }

                let mut slice = array.slice_mut(s!(y * 128..end_y, x * 128..end_x));
                let mut slice_g = array_g.slice_mut(s!(y * 128..end_y, x * 128..end_x));
                let mut slice_b = array_b.slice_mut(s!(y * 128..end_y, x * 128..end_x));
                // break;

                Zip::from(&mut slice).and(&block_arr).for_each(|a, &bb| {
                    if bb == val_value {
                        *a = r;
                    }
                });

                // let block_arr = block_arr.clone();
                Zip::from(&mut slice_g).and(&block_arr).for_each(|a, &bb| {
                    if bb == val_value {
                        *a = g;
                    }
                });

                Zip::from(&mut slice_b).and(&block_arr).for_each(|a, &bb| {
                    if bb == val_value {
                        *a = b;
                    }
                });
            }
            // break;
        }

        let buffer_r = Buffer::new((x_size, y_size), array.into_raw_vec());
        let buffer_g = Buffer::new((x_size, y_size), array_g.into_raw_vec());
        let buffer_b = Buffer::new((x_size, y_size), array_b.into_raw_vec());

        // Output
        {
            let driver = Driver::get("GTiff").unwrap();
            let mut new_ds = driver
                .create_with_band_type::<T>(out_raster, rb.x_size() as isize, rb.y_size() as isize, 3)
                .unwrap();
            new_ds
                .set_geo_transform(&ds.geo_transform().unwrap())
                .unwrap();
            new_ds.set_spatial_ref(&ds.spatial_ref().unwrap()).unwrap();

            let mut new_rb = new_ds.rasterband(1).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb.write((0, 0), (x_size, y_size), &buffer_r).unwrap();
            // band 2
            let mut new_rb = new_ds.rasterband(2).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb.write((0, 0), (x_size, y_size), &buffer_g).unwrap();

            // band 3
            let mut new_rb = new_ds.rasterband(3).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb.write((0, 0), (x_size, y_size), &buffer_b).unwrap();
        }
    }
}

/// Parse the input parameters into this struct.
pub struct Config {
    in_raster: String,
    out_raster: String,
    valid_value: f64,
    r: u8,
    g: u8,
    b: u8,
    n_threads: usize,
}

impl Config {
    /// Parse the input arguments passed in by clap.
    pub fn new(matches: clap::ArgMatches) -> Result<Config, &'static str> {
        let mut in_raster = String::new();
        if let Some(param) = matches.value_of("in_raster") {
            in_raster = String::from(param);
        }
        let mut out_raster = String::new();
        if let Some(param) = matches.value_of("out_raster") {
            out_raster = String::from(param);
        }
        let mut valid_value: f64 = 1.;
        if let Some(param) = matches.value_of("valid") {
            valid_value = param.trim().parse().unwrap();
        }
        let (mut r, mut g, mut b) = (0, 0, 0);
        if let Some(param) = matches.value_of("rgb") {
            let mut rgb = param.trim().split_whitespace();
            r = rgb
                .next()
                .expect("Failed parsing rgb value")
                .parse()
                .expect("Failed parsing r from rgb.");
            g = rgb
                .next()
                .expect("Failed parsing rgb value")
                .parse()
                .expect("Failed parsing g from rgb.");
            b = rgb
                .next()
                .expect("Failed parsing rgb value")
                .parse()
                .expect("Failed parsing b from rgb.");
        }
        let mut n_threads: usize = 0;
        if let Some(param) = matches.value_of("n_threads") {
            n_threads = param.trim().parse().unwrap();
        }

        Ok(Config {
            in_raster,
            out_raster,
            valid_value,
            r,
            g,
            b,
            n_threads,
        })
    }
}

/// Main program to run.
pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let in_raster = config.in_raster;
    let out_raster = config.out_raster;

    let val_value: f64 = config.valid_value;
    let (r, g, b) = (config.r, config.g, config.b);
    let ds = Dataset::open(Path::new(&in_raster)).expect("Open the input raster fail!");
    let count = ds.raster_count();
    if count != 1 {
        println!("Warning: input raster has more than 1 ({}) band", &count);
        println!("Try using the first band.");
    }
    let rb = ds
        .rasterband(1)
        .expect("Fetch the first band of the input fail!");
    // let (x_size, y_size) = ds.raster_size();
    let dtype = rb.band_type();

    match dtype {
        1 => {
            type T = u8;  //  GDALDataType::GDT_Byte
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        2 => {
            type T = u16;  // GDALDataType::GDT_UInt16
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        3 => {
            type T = i16;  // GDALDataType::GDT_Int16
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        4 => {
            type T = u32;  // GDALDataType::GDT_UInt32
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        5 => {
            type T = i32;  // GDALDataType::GDT_Int32
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        6 => {
            type T = f32;  // GDALDataType::GDT_Float32
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        7 => {
            type T = f64;  // GDALDataType::GDT_Float64
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        _ => (),
    }

    Ok(())
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

pub fn gray2rgb<T: GdalType + Copy + PartialEq + Into<f64>>(
    rb: RasterBand,
    out_raster: &str,
    ds: &Dataset,
    val_value: T,
    r: u8, g: u8, b: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let rb_data_buffer: Buffer<T> = rb.read_band_as().unwrap();
    let rb_data = rb_data_buffer.data;
    let (x_size, y_size) = ds.raster_size();
    let new_buff_data = vec![0 as u8; x_size*y_size];
    let mut rb_data_new: Buffer<u8> = Buffer::new((x_size, y_size), new_buff_data);
    
    // Output
    {
        let driver = Driver::get("GTiff").unwrap();
        let mut new_ds = driver
            .create_with_band_type::<u8>(out_raster, x_size as isize, y_size as isize, 3)
            .unwrap();
        new_ds
            .set_geo_transform(&ds.geo_transform().unwrap())
            .unwrap();
        new_ds.set_spatial_ref(&ds.spatial_ref().unwrap()).unwrap();
        // band 1
        // for each
        for (i, d) in (&mut rb_data_new.data).iter_mut().enumerate() {
            if &rb_data[i] == &val_value {
                *d = r;
            }
        }
        let mut new_rb = new_ds.rasterband(1).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb.write((0, 0), (x_size, y_size), &rb_data_new).unwrap();
        // band 2
        // for each
        for (i, d) in (&mut rb_data_new.data).iter_mut().enumerate() {
            if &rb_data[i] == &val_value {
                *d = g;
            }
        }
        let mut new_rb = new_ds.rasterband(2).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb.write((0, 0), (x_size, y_size), &rb_data_new).unwrap();

        // band 3
        // for each
        for (i, d) in (&mut rb_data_new.data).iter_mut().enumerate() {
            if &rb_data[i] == &val_value {
                *d = b;
            }
        }
        let mut new_rb = new_ds.rasterband(3).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb.write((0, 0), (x_size, y_size), &rb_data_new).unwrap();
    }
    Ok(())
}

pub fn run1(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let in_raster = config.in_raster;
    let out_raster = config.out_raster;

    // let val_value: f64 = config.valid_value;
    let val_value: u8 = config.valid_value as u8;
    let (r, g, b) = (config.r, config.g, config.b);
    let ds = Dataset::open(Path::new(&in_raster)).expect("Open the input raster fail!");
    let count = ds.raster_count();
    if count != 1 {
        println!("Warning: input raster has more than 1 ({}) band", &count);
        println!("Try using the first band.");
    }
    let rb = ds
        .rasterband(1)
        .expect("Fetch the first band of the input fail!");
    // let (x_size, y_size) = ds.raster_size();
    // let dtype = rb.band_type();
    let (x_size, y_size) = ds.raster_size();

    let (block_x, block_y) = rb.block_size();
    let (x_remain, _) = rb.actual_block_size(((x_size / block_x) as isize, 0)).unwrap();
    let (_, y_remain) = rb.actual_block_size((0, (y_size / block_y) as isize)).unwrap();
    type T = u8;

    let array = Array2::<T>::zeros((y_size, x_size));
    let array_g = array.clone();
    let array_b = array.clone();

    let array_r = Arc::new(Mutex::new(array.to_shared()));
    let array_g = Arc::new(Mutex::new(array_g.to_shared()));
    let array_b = Arc::new(Mutex::new(array_b.to_shared()));

    let pool = ThreadPool::new(config.n_threads);
    for x in 0..=x_size / block_x {
        for y in 0..=y_size / block_y {
            let arr_r = Arc::clone(&array_r);
            let arr_g = Arc::clone(&array_g);
            let arr_b = Arc::clone(&array_b);
            let block_arr: Array2<T> = rb.read_block((x, y)).unwrap();
            
            pool.execute(move || {
                write_block_thread(block_arr, x, y, x_size, y_size, block_x, block_y,
                    x_remain, y_remain, arr_r, arr_g, arr_b,
                    val_value, r, g, b);
            });
            
        }
    }
    pool.join();

    let buffer_r = Buffer::new((x_size, y_size), array_r.lock().unwrap().to_owned().into_raw_vec());
    let buffer_g = Buffer::new((x_size, y_size), array_g.lock().unwrap().to_owned().into_raw_vec());
    let buffer_b = Buffer::new((x_size, y_size), array_b.lock().unwrap().to_owned().into_raw_vec());

    use std::thread;
    // Output
    println!("Writing images");
    {
        let driver = Driver::get("GTiff").unwrap();
        let mut new_ds = driver
            .create_with_band_type::<T>(&out_raster, rb.x_size() as isize, rb.y_size() as isize, 3)
            .unwrap();
        new_ds
            .set_geo_transform(&ds.geo_transform().unwrap())
            .unwrap();
        new_ds.set_spatial_ref(&ds.spatial_ref().unwrap()).unwrap();

        let new_ds = Arc::new(Mutex::new(new_ds));

        let b1 = Arc::clone(&new_ds);
        let b2 = Arc::clone(&new_ds);
        let b3 = Arc::clone(&new_ds);
        let mut handls = vec![];
        let jh1 = thread::spawn(move || {
            let ds = b1.lock().unwrap();
            let mut new_rb = ds.rasterband(1).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb.write((0, 0), (x_size, y_size), &buffer_r).unwrap();
        });
        handls.push(jh1);
        let jh2 = thread::spawn(move || {
            let ds = b2.lock().unwrap();
            let mut new_rb = ds.rasterband(2).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb.write((0, 0), (x_size, y_size), &buffer_g).unwrap();
        });
        handls.push(jh2);
        let jh3 = thread::spawn(move || {
            let ds = b3.lock().unwrap();
            let mut new_rb = ds.rasterband(3).unwrap();
            new_rb.set_no_data_value(0.).unwrap();
            new_rb.write((0, 0), (x_size, y_size), &buffer_b).unwrap();
        });
        handls.push(jh3);
        for h in handls {
            h.join().unwrap();
        }
    }

    Ok(())
}

pub fn write_block_thread<T: Copy + PartialEq + Into<f64>>(block_arr: Array2<T>,
    x: usize, y: usize, x_size: usize, y_size: usize, block_x: usize, block_y: usize,
    x_remain: usize, y_remain: usize,
    array_r: Arc<Mutex<ArcArray<u8, Dim<[usize; 2]>>>>, array_g: Arc<Mutex<ArcArray<u8, Dim<[usize; 2]>>>>, array_b: Arc<Mutex<ArcArray<u8, Dim<[usize; 2]>>>>,
    val_value: T, r: u8, g: u8, b: u8) {

        let mut r_arr = array_r.lock().unwrap();
        let mut g_arr = array_g.lock().unwrap();
        let mut b_arr = array_b.lock().unwrap();

        write_block(block_arr, x, y, x_size, y_size, block_x, block_y, x_remain, y_remain, &mut r_arr, &mut g_arr, &mut b_arr, val_value, r, g, b);
    }

pub fn write_block<T: Copy + PartialEq + Into<f64>>(mut block_arr: Array2<T>,
    x: usize, y: usize, x_size: usize, y_size: usize, block_x: usize, block_y: usize,
    x_remain: usize, y_remain: usize,
    array_r: &mut ArcArray<u8, Dim<[usize; 2]>>, array_g: &mut ArcArray<u8, Dim<[usize; 2]>>, array_b: &mut ArcArray<u8, Dim<[usize; 2]>>,
    val_value: T, r: u8, g: u8, b: u8) {
    // block index
    let block_x_e = block_x * (x + 1);
    let block_y_e = block_y * (y + 1);
    let end_x = if block_x_e >= x_size {
        x_size
    } else {
        block_x_e
    };
    let end_y = if block_y_e >= y_size {
        y_size
    } else {
        block_y_e
    };
    if block_x_e >= x_size && !(block_y_e >= y_size) {
        block_arr = block_arr.slice(s!(0..block_y, 0..x_remain)).to_owned();
    }
    if block_x_e < x_size && block_y_e >= y_size {
        block_arr = block_arr.slice(s!(0..y_remain, 0..block_x)).to_owned();
    }
    if block_x_e >= x_size && block_y_e >= y_size {
        block_arr = block_arr.slice(s!(0..y_remain, 0..x_remain)).to_owned();
    }

    // let (index0, index1) = (y * block_y..end_y, x * block_x..end_x);
    let mut slice = array_r.slice_mut(s!(y * block_y..end_y, x * block_x..end_x));
    let mut slice_g = array_g.slice_mut(s!(y * block_y..end_y, x * block_x..end_x));
    let mut slice_b = array_b.slice_mut(s!(y * block_y..end_y, x * block_x..end_x));
    // break;

    Zip::from(&mut slice).and(&block_arr).for_each(|a, &bb| {
        if bb == val_value {
            *a = r;
        }
    });

    // let block_arr = block_arr.clone();
    Zip::from(&mut slice_g).and(&block_arr).for_each(|a, &bb| {
        if bb == val_value {
            *a = g;
        }
    });

    Zip::from(&mut slice_b).and(&block_arr).for_each(|a, &bb| {
        if bb == val_value {
            *a = b;
        }
    });
}
