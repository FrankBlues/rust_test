use gdal::raster::{Buffer, GdalType, RasterBand};
use gdal::{Dataset, Driver};
use std::path::Path;
use gdal_test::gray2rgb;
use ndarray::{s, Array2, Zip};
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
    println!("{:?}, {:?}", x_size, y_size);
    let dtype = rb.band_type();
    let (block_x, block_y) = rb.block_size();
    println!("{:?}", rb.block_size());
    let (x_remain, _) = rb.actual_block_size(((x_size / block_x) as isize, 0)).unwrap();
    let (_, y_remain) = rb.actual_block_size((0, (y_size / block_y) as isize)).unwrap();
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
            let block_arr: Array2<u8> = rb.read_block((x, y)).unwrap();
            write_block(block_arr, x, y, x_size, y_size, block_x, block_y,
                x_remain, y_remain, &mut array, &mut array_g, &mut array_b,
                val_value, r, g, b);
        }
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

fn write_block(mut block_arr: Array2<u8>,
    x: usize, y: usize, x_size: usize, y_size: usize, block_x: usize, block_y: usize,
    x_remain: usize, y_remain: usize,
    array_r: &mut Array2<u8>, array_g: &mut Array2<u8>, array_b: &mut Array2<u8>,
    val_value: u8, r: u8, g: u8, b: u8) {
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
