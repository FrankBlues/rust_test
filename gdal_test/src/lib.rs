use gdal::raster::{Buffer, GDALDataType, GdalType, RasterBand};
use gdal::spatial_ref::SpatialRef;
use gdal::{Dataset, Driver};
use ndarray::{s, ArcArray, Array2, Dim, Zip};
use std::path::Path;

use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

mod rtree;
pub use rtree::{run_find_intersected, ImageBoundary, ParamsFindIntersected};

mod io_utils;
pub use io_utils::{check_parent_dir, get_files, write_vec_to_text};

mod window;
pub use window::calculate_window;

mod merge;
pub use merge::merge;

// mod warp;
// pub use warp::raster_projection::reproject;
// #[cfg(feature = "ndarray")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_value() {
        let dataset = Dataset::open(Path::new("/data/GF2_PMS1_E108.9_N34.2_20181026_L1A0003549596-MSS11.tif")).unwrap();
        let metas = RasterMetadata::from(&dataset);
        assert_eq!(metas.driver, String::from("GTiff"));
        
        assert_eq!(xy2pixel_xy(108.758711, 34.330951, &metas.geo_transform), (2197, 594));
        let data = dataset.rasterband(3).unwrap().read_as::<u16>((2197, 594), (1, 1), (1, 1), None).unwrap();
        assert_eq!(data.data[0], 269);

        let (cols, rows) = (metas.cols, metas.rows);
        let data = dataset.rasterband(3).unwrap().read_band_as::<u16>().unwrap();
        assert_eq!(data.data[2197-1 + 594*rows], 269);

        let (x, y) = pixel_xy2xy(6471, 1759, &metas.geo_transform);
        assert_eq!(xy2pixel_xy(x, y, &metas.geo_transform), (6471, 1759));
        let data = dataset.rasterband(1).unwrap().read_as::<u16>((6471, 1759), (1, 1), (1, 1), None).unwrap();
        assert_eq!(data.data[0], 379);

        let data = dataset.rasterband(1).unwrap().read_band_as::<u16>().unwrap();
        assert_eq!(data.size, (cols, rows));
        // wrong
        // assert_eq!(data.data[6471-1 + 1759*rows], 379);

        let dataset = Dataset::open(Path::new("/data/range100.tif")).unwrap(); 
        let metas = RasterMetadata::from(&dataset);

        let data = dataset.rasterband(1).unwrap().read_as::<u8>((4, 3), (1, 1), (1, 1), None).unwrap();
        assert_eq!(data.data[0], 34);

        let data = dataset.rasterband(1).unwrap().read_band_as::<u8>().unwrap();
        assert_eq!(data.data[3 * metas.rows + 4], 34);
        assert_eq!(pixel_value::<u8>(&dataset, 4, 5, None), vec![54]);
        assert_eq!(pixel_value::<u8>(&dataset, 4, 5, Some(vec![1])), vec![54]);
    }
}

/// return the raster boundary [left, bottom, right, top]
pub fn raster_boundary(geo_transform: &[f64; 6], raster_size: &(usize, usize)) -> [f64; 4] {
    let res = geo_transform[1];
    let bottom = geo_transform[3] - res * (raster_size.1 as f64);
    let right = geo_transform[0] + res * (raster_size.0 as f64);

    [geo_transform[0], bottom, right, geo_transform[3]]
}

/// Get the rows and cols of the pixels containing (x, y)
pub fn xy2pixel_xy(x: f64, y: f64, geo_transform: &[f64; 6]) -> (usize, usize) {
    let res = geo_transform[1];
    (
        ((x - geo_transform[0]) / res + 0.5).floor() as usize,
        ((geo_transform[3] - y) / res + 0.5).floor() as usize,
    )
}

/// Get the x and y coordinates of pixels at `pixel_x` and `pixel_y`
pub fn pixel_xy2xy(pixel_x: usize, pixel_y: usize, geo_transform: &[f64; 6]) -> (f64, f64) {
    let res = geo_transform[1];
    (
        geo_transform[0] + (pixel_x as f64) * res,
        geo_transform[3] - (pixel_y as f64) * res,
    )
}

/// Get pixel value
pub fn pixel_value<T: GdalType + Copy>(
    dataset: &Dataset, pixel_x: usize, pixel_y: usize, band_index: Option<Vec<isize>>
) -> Vec<T> {
    let mut vals: Vec<T> = Vec::new();
    let mut _idx: Vec<isize> = Vec::new();
    match band_index {
        Some(idx) => _idx = idx,
        None => _idx = (1..=dataset.raster_count()).collect::<Vec<isize>>()
    }
    for i in _idx {
        let data = dataset.rasterband(i).unwrap().read_as::<T>(
            (pixel_x as isize, pixel_y as isize), (1, 1), (1, 1), None).unwrap();
        vals.push(data.data[0]);
    }
    vals
}

/// Parse raster metadatas
#[derive(Clone)]
pub struct RasterMetadata {
    pub geo_transform: [f64; 6],
    pub rows: usize,
    pub cols: usize,
    pub res: (f64, f64),
    pub count: isize,
    pub bounds: [f64; 4],
    pub nodata: Option<f64>,
    pub dtype: GDALDataType::Type,
    pub driver: String,
    pub srs: Option<SpatialRef>,
}

impl RasterMetadata {
    pub fn new() -> RasterMetadata {
        RasterMetadata {
            geo_transform: [0.0; 6],
            rows: 0,
            cols: 0,
            res: (0.0, 0.0),
            count: 1,
            bounds: [0.0; 4],
            nodata: None,
            dtype: 1,
            driver: String::from("GTiff"),
            srs: None,
        }
    }
    pub fn from(dataset: &Dataset) -> RasterMetadata {
        let mut geo_transform = [0.0; 6];
        match dataset.geo_transform() {
            Ok(gt) => geo_transform = gt,
            Err(e) => {
                println!("no geo tranform! error: {}", e);
                geo_transform[1] = 1.0;
                geo_transform[5] = -1.0
            }
        }
        let (cols, rows) = dataset.raster_size();
        let srs;
        match dataset.spatial_ref() {
            Ok(sr) => srs = Some(sr),
            Err(_) => srs = None,
        }
        let res = (geo_transform[1], -geo_transform[5]);
        let count = dataset.raster_count();
        let first_band = dataset.rasterband(1).unwrap();
        let nodata = first_band.no_data_value();
        let dtype = first_band.band_type();
        RasterMetadata {
            geo_transform: geo_transform,
            rows: rows,
            cols: cols,
            res: res,
            count: count,
            bounds: raster_boundary(&geo_transform, &(cols, rows)),
            nodata: nodata,
            dtype: dtype,
            driver: dataset.driver().short_name(),
            srs: srs,
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

/// Main program(gray2rgb) to run.
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
            type T = u8; //  GDALDataType::GDT_Byte
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        2 => {
            type T = u16; // GDALDataType::GDT_UInt16
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        3 => {
            type T = i16; // GDALDataType::GDT_Int16
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        4 => {
            type T = u32; // GDALDataType::GDT_UInt32
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        5 => {
            type T = i32; // GDALDataType::GDT_Int32
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        6 => {
            type T = f32; // GDALDataType::GDT_Float32
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        7 => {
            type T = f64; // GDALDataType::GDT_Float64
            if let Err(e) = gray2rgb::<T>(rb, &out_raster, &ds, val_value as T, r, g, b) {
                eprintln!("GRAY TO RGB error: {}", e);
            }
        }
        _ => (),
    }

    Ok(())
}

pub fn gray2rgb<T: GdalType + Copy + PartialEq + Into<f64>>(
    rb: RasterBand,
    out_raster: &str,
    ds: &Dataset,
    val_value: T,
    r: u8,
    g: u8,
    b: u8,
) -> Result<(), Box<dyn std::error::Error>> {
    let rb_data_buffer: Buffer<T> = rb.read_band_as().unwrap();
    let rb_data = rb_data_buffer.data;
    let (x_size, y_size) = ds.raster_size();
    let new_buff_data = vec![0 as u8; x_size * y_size];
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
        new_rb
            .write((0, 0), (x_size, y_size), &rb_data_new)
            .unwrap();
        // band 2
        // for each
        for (i, d) in (&mut rb_data_new.data).iter_mut().enumerate() {
            if &rb_data[i] == &val_value {
                *d = g;
            }
        }
        let mut new_rb = new_ds.rasterband(2).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb
            .write((0, 0), (x_size, y_size), &rb_data_new)
            .unwrap();

        // band 3
        // for each
        for (i, d) in (&mut rb_data_new.data).iter_mut().enumerate() {
            if &rb_data[i] == &val_value {
                *d = b;
            }
        }
        let mut new_rb = new_ds.rasterband(3).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb
            .write((0, 0), (x_size, y_size), &rb_data_new)
            .unwrap();
    }
    Ok(())
}

/// gray2rgb_block(only u8 type)
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
    let (x_remain, _) = rb
        .actual_block_size(((x_size / block_x) as isize, 0))
        .unwrap();
    let (_, y_remain) = rb
        .actual_block_size((0, (y_size / block_y) as isize))
        .unwrap();
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
                write_block_thread(
                    block_arr, x, y, x_size, y_size, block_x, block_y, x_remain, y_remain, arr_r,
                    arr_g, arr_b, val_value, r, g, b,
                );
            });
        }
    }
    pool.join();

    let buffer_r = Buffer::new(
        (x_size, y_size),
        array_r.lock().unwrap().to_owned().into_raw_vec(),
    );
    let buffer_g = Buffer::new(
        (x_size, y_size),
        array_g.lock().unwrap().to_owned().into_raw_vec(),
    );
    let buffer_b = Buffer::new(
        (x_size, y_size),
        array_b.lock().unwrap().to_owned().into_raw_vec(),
    );

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

pub fn write_block_thread<T: Copy + PartialEq + Into<f64>>(
    block_arr: Array2<T>,
    x: usize,
    y: usize,
    x_size: usize,
    y_size: usize,
    block_x: usize,
    block_y: usize,
    x_remain: usize,
    y_remain: usize,
    array_r: Arc<Mutex<ArcArray<u8, Dim<[usize; 2]>>>>,
    array_g: Arc<Mutex<ArcArray<u8, Dim<[usize; 2]>>>>,
    array_b: Arc<Mutex<ArcArray<u8, Dim<[usize; 2]>>>>,
    val_value: T,
    r: u8,
    g: u8,
    b: u8,
) {
    let mut r_arr = array_r.lock().unwrap();
    let mut g_arr = array_g.lock().unwrap();
    let mut b_arr = array_b.lock().unwrap();

    write_block(
        block_arr, x, y, x_size, y_size, block_x, block_y, x_remain, y_remain, &mut r_arr,
        &mut g_arr, &mut b_arr, val_value, r, g, b,
    );
}

pub fn write_block<T: Copy + PartialEq + Into<f64>>(
    mut block_arr: Array2<T>,
    x: usize,
    y: usize,
    x_size: usize,
    y_size: usize,
    block_x: usize,
    block_y: usize,
    x_remain: usize,
    y_remain: usize,
    array_r: &mut ArcArray<u8, Dim<[usize; 2]>>,
    array_g: &mut ArcArray<u8, Dim<[usize; 2]>>,
    array_b: &mut ArcArray<u8, Dim<[usize; 2]>>,
    val_value: T,
    r: u8,
    g: u8,
    b: u8,
) {
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
