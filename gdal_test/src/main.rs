use gdal::raster::Buffer;
use gdal::{Dataset, Driver};
use std::path::Path;
use gdal_sys::GDALDataType;
// #[cfg(feature = "ndarray")]
use ndarray::Array2;

fn main() {
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
    // type T = u8;
    // match dtype {
    //     1 => {type T = u8},
    //     _ => (),
    // }

    type T= u8;

    let mut rb_data: Buffer<T> = rb.read_band_as().unwrap();
    // let arr = Array2::from_shape_vec(
    //     (y_size, x_size),
    //     rb_data.data,
    // ).unwrap();
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
        // rb_data.data.into_iter().map(|d| if d == val_value {d = r;});
        for d in &mut rb_data.data {
            if *d == val_value {
                *d = r;
            }
        }
        let mut new_rb = new_ds.rasterband(1).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb
            .write((0, 0), (x_size, y_size), &rb_data)
            .unwrap();
        // band 2
        for d in &mut rb_data.data {
            if *d == r {
                *d = g;
            }
        }
        let mut new_rb = new_ds.rasterband(2).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb
            .write((0, 0), (x_size, y_size), &rb_data)
            .unwrap();

        // band 3
        for d in &mut rb_data.data {
            if *d == g {
                *d = b;
            }
        }
        let mut new_rb = new_ds.rasterband(3).unwrap();
        new_rb.set_no_data_value(0.).unwrap();
        new_rb
            .write((0, 0), (x_size, y_size), &rb_data)
            .unwrap();
    }
}
