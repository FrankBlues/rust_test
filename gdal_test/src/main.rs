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


extern crate xml;
use std::io::BufReader;
use std::fs::File;

use xml::reader::{EventReader, XmlEvent};
use std::collections::HashMap;

fn parse_boundaries_from_xml(xml_file: &str, ) -> [f64; 4] {
    let file = File::open(xml_file).expect("Open xml file error.");
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<f64> = Vec::new();
    let elements = vec!["TopLeftLatitude", "TopLeftLongitude", "TopRightLatitude", "TopRightLongitude",
                        "BottomRightLatitude", "BottomRightLongitude", "BottomLeftLatitude", "BottomLeftLongitude"];
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if (&elements).contains(&name.local_name.as_str()) {
                    keys.push(name.local_name);
                }
            }
            Ok(XmlEvent::Characters(chars)) => {
                if values.len() != keys.len() {
                    values.push(chars.parse::<f64>().unwrap());
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
        // break;
    }
    let map: HashMap<String, f64> = keys.into_iter().zip(values.into_iter()).collect();
    let left = map.get("TopLeftLongitude").unwrap().min(*map.get("BottomLeftLongitude").unwrap());
    let bottom = map.get("BottomRightLatitude").unwrap().min(*map.get("BottomLeftLatitude").unwrap());
    let right = map.get("TopRightLongitude").unwrap().max(*map.get("BottomRightLongitude").unwrap());
    let top = map.get("TopLeftLatitude").unwrap().max(*map.get("TopRightLatitude").unwrap());
    [left, bottom, right, top]
}

fn main() {
    // let mut bounds: HashMap<String, f64> = HashMap::new();
    
    let xml_file = "/data/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601-PAN2.xml";
    let file = File::open(xml_file).expect("Open xml file error.");
    let file = BufReader::new(file);

    let parser = EventReader::new(file);

    let mut keys: Vec<String> = Vec::new();
    let mut values: Vec<f64> = Vec::new();
    let elements = vec!["TopLeftLatitude", "TopLeftLongitude", "TopRightLatitude", "TopRightLongitude",
                        "BottomRightLatitude", "BottomRightLongitude", "BottomLeftLatitude", "BottomLeftLongitude"];
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, .. }) => {
                if (&elements).contains(&name.local_name.as_str()) {
                    keys.push(name.local_name);
                }
            }
            Ok(XmlEvent::Characters(chars)) => {
                if values.len() != keys.len() {
                    values.push(chars.parse::<f64>().unwrap());
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
        // break;
    }
    let map: HashMap<String, f64> = keys.into_iter().zip(values.into_iter()).collect();
    let left = map.get("TopLeftLongitude").unwrap().min(*map.get("BottomLeftLongitude").unwrap());
    let bottom = map.get("BottomRightLatitude").unwrap().min(*map.get("BottomLeftLatitude").unwrap());
    let right = map.get("TopRightLongitude").unwrap().max(*map.get("BottomRightLongitude").unwrap());
    let top = map.get("TopLeftLatitude").unwrap().max(*map.get("TopRightLatitude").unwrap());
    println!("{:?}", [left, bottom, right, top]);

    // let in_raster = "/data/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601-PAN2.tiff";
    // let out_raster = "/data/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601-PAN2.tiff";

    // let val_value: u8 = 158;
    // let (r, g, b) = (22 as u8, 181 as u8, 255 as u8);
    // let ds = Dataset::open(Path::new(in_raster)).unwrap();
    // println!("{:?}", ds.raster_size());
    // match ds.geo_transform() {
    //     Ok(trans) => println!("{:?}", trans),
    //     Err(e) => eprintln!("Cannot get geotransform, try from xml error is {}", e),
    // }
    // println!("{:?}", ds.geo_transform());
    // let src_sr = ds.spatial_ref().unwrap();
    // let count = ds.raster_count();
    // if count != 1 {
    //     println!("Warning: input raster has more than 1 ({}) band", &count);
    //     println!("Try using the first band.");
    // }
    // let rb = ds.rasterband(1).unwrap();
    // let (x_size, y_size) = ds.raster_size();
    // println!("{:?}, {:?}", x_size, y_size);
    // let dtype = rb.band_type();
    // type T = u8;
    // let (block_x, block_y) = rb.block_size();
    // println!("{:?}", rb.block_size());
    // let (x_remain, _) = rb.actual_block_size(((x_size / block_x) as isize, 0)).unwrap();
    // let (_, y_remain) = rb.actual_block_size((0, (y_size / block_y) as isize)).unwrap();
    // let prj = ds.projection();
    // let dst_wkt = "PROJCS[\"WGS 84 / UTM zone 48N\",GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.0174532925199433,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]],PROJECTION[\"Transverse_Mercator\"],PARAMETER[\"latitude_of_origin\",0],PARAMETER[\"central_meridian\",105],PARAMETER[\"scale_factor\",0.9996],PARAMETER[\"false_easting\",500000],PARAMETER[\"false_northing\",0],UNIT[\"metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],AXIS[\"Easting\",EAST],AXIS[\"Northing\",NORTH],AUTHORITY[\"EPSG\",\"32648\"]]";

    // let driver = Driver::get("GTiff").unwrap();
    // let mut new_ds = driver.create_with_band_type::<u8>("/data/test_proj.tif", x_size as isize, y_size as isize, count).unwrap();
    // let dst_sr = SpatialRef::from_epsg(32648).unwrap();
    // let dst_wkt = dst_sr.to_wkt().unwrap();
    // println!("{}", dst_wkt);
    // new_ds.set_spatial_ref(&dst_sr).unwrap();
    // // new_ds.set_no_data_value(0.).unwrap();
    // new_ds.set_geo_transform(&[0.; 6]).unwrap();

    // let mut x = [109., 108.];
    // let mut y = [34.5, 35.1];
    // let mut z = [0., 0.];

    // println!("{:?},{:?},{:?}", x, y, z);
    // let trans = CoordTransform::new(&src_sr, &dst_sr).unwrap();
    // trans.transform_coords(&mut x, &mut y, &mut z).unwrap();
    // println!("{:?},{:?},{:?}", x, y, z);

}


// use std::ffi::CString;

// fn reproject1(src: &Dataset, dst: &str, dst_prj: &str) -> Result<()> {
//     let rv = unsafe {
//         gdal_sys::GDALCreateAndReprojectImage(
//             src.c_dataset(),
//             null(),
//             CString::new(dst).unwrap().as_ptr(),
//             CString::new(dst_prj).unwrap().as_ptr(),
//             null_mut(),
//             // Driver::get("GTiff").unwrap().c_driver(),
//             null_mut(),
//             GDALResampleAlg::GRA_Bilinear,
//             0.0,
//             0.0,
//             None,
//             null_mut(),
//             null_mut(),
//         )
//     };
//     if rv != CPLErr::CE_None {
//         return Err(_last_cpl_err(rv));
//     }
//     Ok(())
// }



// fn reproject(src: &Dataset, dst: &Dataset) -> Result<()> {
//     let rv = unsafe {
//         gdal_sys::GDALReprojectImage(
//             src.c_dataset(),
//             null(),
//             dst.c_dataset(),
//             null(),  // CString::new(dst_prj).unwrap().as_ptr(),
//             GDALResampleAlg::GRA_Bilinear,
//             0.0,
//             0.0,
//             None,
//             null_mut(),
//             null_mut(),
//         )
//     };
//     if rv != CPLErr::CE_None {
//         return Err(_last_cpl_err(rv));
//     }
//     Ok(())
// }

// fn _last_cpl_err(cpl_err_class: CPLErr::Type) -> GdalError {
//     let last_err_no = unsafe { gdal_sys::CPLGetLastErrorNo() };
//     let last_err_msg = _string(unsafe { gdal_sys::CPLGetLastErrorMsg() });
//     unsafe { gdal_sys::CPLErrorReset() };
//     GdalError::CplError {
//         class: cpl_err_class,
//         number: last_err_no,
//         msg: last_err_msg,
//     }
// }

// pub fn _string(raw_ptr: *const c_char) -> String {
//     let c_str = unsafe { CStr::from_ptr(raw_ptr) };
//     c_str.to_string_lossy().into_owned()
// }