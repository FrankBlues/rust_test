use gdal::raster::{Buffer, GdalType, RasterBand};
use gdal::{Dataset, Driver};
use std::path::Path;
use gdal_test::{gray2rgb, write_block_thread};
use ndarray::{s, Array2, Zip, ArcArray, Dim};

extern crate threadpool;
use threadpool::ThreadPool;
use std::sync::{Mutex, Arc};

use gdal_sys::GDALReprojectImage;
use gdal_sys::{self, CPLErr, GDALResampleAlg};
use std::ptr::{null, null_mut};
use gdal::errors::*;
use std::ffi::CStr;

use libc::c_char;

use gdal::spatial_ref::SpatialRef;
use gdal::spatial_ref::CoordTransform;
use gdal_sys::{GDALWarpOperationH, GDALCreateWarpOperation, GDALWarpOptions, GDALCreateWarpOptions};

fn main() {
    let in_raster = "/data/read_write_raster/road.tif";
    let out_raster = "/data/trans_raster.tif";

    let val_value: u8 = 158;
    let (r, g, b) = (22 as u8, 181 as u8, 255 as u8);
    let ds = Dataset::open(Path::new(in_raster)).unwrap();
    let src_sr = ds.spatial_ref().unwrap();
    let count = ds.raster_count();
    if count != 1 {
        println!("Warning: input raster has more than 1 ({}) band", &count);
        println!("Try using the first band.");
    }
    let rb = ds.rasterband(1).unwrap();
    let (x_size, y_size) = ds.raster_size();
    println!("{:?}, {:?}", x_size, y_size);
    let dtype = rb.band_type();
    type T = u8;
    let (block_x, block_y) = rb.block_size();
    println!("{:?}", rb.block_size());
    let (x_remain, _) = rb.actual_block_size(((x_size / block_x) as isize, 0)).unwrap();
    let (_, y_remain) = rb.actual_block_size((0, (y_size / block_y) as isize)).unwrap();
    let prj = ds.projection();
    let dst_proj = "PROJCS[\"WGS 84 / UTM zone 48N\",GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.0174532925199433,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]],PROJECTION[\"Transverse_Mercator\"],PARAMETER[\"latitude_of_origin\",0],PARAMETER[\"central_meridian\",105],PARAMETER[\"scale_factor\",0.9996],PARAMETER[\"false_easting\",500000],PARAMETER[\"false_northing\",0],UNIT[\"metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],AXIS[\"Easting\",EAST],AXIS[\"Northing\",NORTH],AUTHORITY[\"EPSG\",\"32648\"]]";

    
    

    let driver = Driver::get("GTiff").unwrap();
    let mut new_ds = driver.create_with_band_type::<u8>("/data/test_proj.tif", x_size as isize, y_size as isize, count).unwrap();
    let dst_sr = SpatialRef::from_epsg(32648).unwrap();
    new_ds.set_spatial_ref(&dst_sr).unwrap();
    // new_ds.set_no_data_value(0.).unwrap();
    new_ds.set_geo_transform(&[0.; 6]).unwrap();

    let mut x = [109., 108.];
    let mut y = [34.5, 35.1];
    let mut z = [0., 0.];

    println!("{:?},{:?},{:?}", x, y, z);
    let trans = CoordTransform::new(&src_sr, &dst_sr).unwrap();
    trans.transform_coords(&mut x, &mut y, &mut z).unwrap();
    println!("{:?},{:?},{:?}", x, y, z);

    let warp_option = GDALCreateWarpOptions();
    // &warp_option
    let o_warper: GDALWarpOperationH = GDALCreateWarpOperation();

    //GDALSuggestedWarpOutput2


    // reproject(&ds, &new_ds).unwrap();
    // reproject1(&ds, "/data/test_proj1.tif", dst_proj).unwrap();
    // let mut new_ds = driver
    //     .create_with_band_type::<T>(out_raster, rb.x_size() as isize, rb.y_size() as isize, 3)
    //     .unwrap();

    // // gray2rgb::<T>(rb, &out_raster, &ds, val_value, r, g, b).unwrap();

    // let array = Array2::<u8>::zeros((y_size, x_size));
    // let array_g = array.clone();
    // let array_b = array.clone();

    // let array_r = Arc::new(Mutex::new(array.to_shared()));
    // let array_g = Arc::new(Mutex::new(array_g.to_shared()));
    // let array_b = Arc::new(Mutex::new(array_b.to_shared()));
    // println!("{:?}", array_r.lock().unwrap().sum());
    // let pool = ThreadPool::new(40);
    // for x in 0..=x_size / block_x {
    //     for y in 0..=y_size / block_y {
    //         let array_r = Arc::clone(&array_r);
    //         let array_g = Arc::clone(&array_g);
    //         let array_b = Arc::clone(&array_b);

    //         let block_arr: Array2<u8> = rb.read_block((x, y)).unwrap();
    //         // write_block(block_arr, x, y, x_size, y_size, block_x, block_y,
    //         //     x_remain, y_remain, array_r, array_g, array_b,
    //         //     val_value, r, g, b);

    //         pool.execute(move || {
    //             write_block_thread(block_arr, x, y, x_size, y_size, block_x, block_y,
    //                     x_remain, y_remain, array_r, array_g, array_b,
    //                     val_value, r, g, b)
    //         });
            
    //     }
    // }
    // pool.join();
    // let buffer_r = Buffer::new((x_size, y_size), array_r.lock().unwrap().to_owned().into_raw_vec());
    // let buffer_g = Buffer::new((x_size, y_size), array_g.lock().unwrap().to_owned().into_raw_vec());
    // let buffer_b = Buffer::new((x_size, y_size), array_b.lock().unwrap().to_owned().into_raw_vec());

    // Output
    // println!("Writing images");
    // {
    //     let driver = Driver::get("GTiff").unwrap();
    //     let mut new_ds = driver
    //         .create_with_band_type::<T>(out_raster, rb.x_size() as isize, rb.y_size() as isize, 3)
    //         .unwrap();
    //     new_ds
    //         .set_geo_transform(&ds.geo_transform().unwrap())
    //         .unwrap();
    //     new_ds.set_spatial_ref(&ds.spatial_ref().unwrap()).unwrap();

    //     let mut new_rb = new_ds.rasterband(1).unwrap();
    //     new_rb.set_no_data_value(0.).unwrap();
    //     new_rb.write((0, 0), (x_size, y_size), &buffer_r).unwrap();
    //     // band 2
    //     let mut new_rb = new_ds.rasterband(2).unwrap();
    //     new_rb.set_no_data_value(0.).unwrap();
    //     new_rb.write((0, 0), (x_size, y_size), &buffer_g).unwrap();

    //     // band 3
    //     let mut new_rb = new_ds.rasterband(3).unwrap();
    //     new_rb.set_no_data_value(0.).unwrap();
    //     new_rb.write((0, 0), (x_size, y_size), &buffer_b).unwrap();
    // }
}


use std::ffi::CString;

fn reproject1(src: &Dataset, dst: &str, dst_prj: &str) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALCreateAndReprojectImage(
            src.c_dataset(),
            null(),
            CString::new(dst).unwrap().as_ptr(),
            CString::new(dst_prj).unwrap().as_ptr(),
            null_mut(),
            // Driver::get("GTiff").unwrap().c_driver(),
            null_mut(),
            GDALResampleAlg::GRA_Bilinear,
            0.0,
            0.0,
            None,
            null_mut(),
            null_mut(),
        )
    };
    if rv != CPLErr::CE_None {
        return Err(_last_cpl_err(rv));
    }
    Ok(())
}



fn reproject(src: &Dataset, dst: &Dataset) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALReprojectImage(
            src.c_dataset(),
            null(),
            dst.c_dataset(),
            null(),  // CString::new(dst_prj).unwrap().as_ptr(),
            GDALResampleAlg::GRA_Bilinear,
            0.0,
            0.0,
            None,
            null_mut(),
            null_mut(),
        )
    };
    if rv != CPLErr::CE_None {
        return Err(_last_cpl_err(rv));
    }
    Ok(())
}

fn _last_cpl_err(cpl_err_class: CPLErr::Type) -> GdalError {
    let last_err_no = unsafe { gdal_sys::CPLGetLastErrorNo() };
    let last_err_msg = _string(unsafe { gdal_sys::CPLGetLastErrorMsg() });
    unsafe { gdal_sys::CPLErrorReset() };
    GdalError::CplError {
        class: cpl_err_class,
        number: last_err_no,
        msg: last_err_msg,
    }
}

pub fn _string(raw_ptr: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(raw_ptr) };
    c_str.to_string_lossy().into_owned()
}