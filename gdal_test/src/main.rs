use gdal::Metadata;
use gdal::raster::{Buffer, GdalType, RasterBand, ResampleAlg};
use gdal::{Dataset, Driver};
use gdal_test::{gray2rgb, write_block_thread};
use ndarray::{s, ArcArray, Array2, Dim, Zip};
use std::path::Path;

extern crate threadpool;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

use gdal::errors::*;
use gdal_sys::GDALReprojectImage;
use gdal_sys::{self, CPLErr, GDALResampleAlg};
use std::ffi::CStr;
use std::ptr::{null, null_mut};

use libc::c_char;

use gdal::spatial_ref::CoordTransform;
use gdal::spatial_ref::SpatialRef;
use gdal_sys::{
    CPLMalloc, CSLFetchNameValue, CSLSetNameValue, GDALApproxTransform,
    GDALApproxTransformerOwnsSubtransformer, GDALChunkAndWarpImage, GDALChunkAndWarpMulti,
    GDALClose, GDALCreate, GDALCreateApproxTransformer, GDALCreateGenImgProjTransformer,
    GDALCreateGenImgProjTransformer2, GDALCreateWarpOperation, GDALCreateWarpOptions,
    GDALDestroyGenImgProjTransformer, GDALGenImgProjTransform, GDALGetCacheMax, GDALGetCacheMax64,
    GDALSetCacheMax, GDALSetCacheMax64, GDALSetGeoTransform, GDALSetProjection,
    GDALSuggestedWarpOutput, GDALTermProgress, GDALTransformerFunc, GDALWarpInitDstNoDataReal,
    GDALWarpInitSrcNoDataReal, GDALWarpOperationH, GDALWarpOptions,
};

use std::fs::File;
use std::io::BufReader;
extern crate xml;
use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};

use std::ffi::CString;

use std::time::SystemTime;
mod warp;
use warp::raster_projection::reproject;

mod io_utils;
use io_utils::get_files;

use gdal_test::{RasterMetadata, raster_boundary};
use std::any::type_name;
use std::any::Any;

enum ResParam {
    One(f64),
    Two(f64, f64),
}

// fn type_of(object: &Any) -> &str{
//     if object.is::<String>() {
//          return "str";
//     } else if object.is::<f64>() {
//         return "f64";
//     } else if object.is::<(f64, f64)>() {
//         return "tuple";
//     }
//     ""
// }
fn main() {
    let raster_dir = "/data/rtree_data/images/tif";
    let in_raster = "/data/rtree_data/images/tif/s_16_xian_prj7.tif";
    // let first_ds = Dataset::open(&Path::new(in_raster)).expect("Open raster file error.");
    // let first_metas = RasterMetadata::from(&first_ds);
    // println!("{:?}", first_metas.nodata);

    // let ds = Dataset::open(Path::new(in_raster)).unwrap();
    // reproject(&ds, 32648, "/data/test_mosaic/s_16_xian_prj7.tif");

    let output_file = "/data/mosaic6.tif";
    let output_count: Option<isize> = None;
    let output_bounds: Option<[f64; 4]> = None;
    let output_res: Option<(f64, f64)> = None;
    // let output_res: Option<(f64, f64)> = Some((3e-5, 4e-5));
    let output_nodata: Option<f64> = None;
    // let output_nodata: Option<f64> = Some(0.0);
    let output_band_index: Option<Vec<isize>> = None;
    
    
    // type(res)
    // match output_res {
    //     ResParam::one => println!(""),
    //     ResParam::two => println!(""),
    // }
    
    // let in_raster = "/data/S2A_MSIL1C_20210901T025551_N0301_R032_T50SLH_20210901T050029_13.jpg";

    let mut files = vec![];
    for entry in get_files("/data/test_mosaic", ".tif").unwrap() {
        match entry {
            Ok(p) => files.push(p.to_str().unwrap().to_owned()),
            Err(e) => println!("{:?}", e),
        }
    }
    println!("{:?}", files);
    // Dataset
    let first_ds = Dataset::open(&Path::new(&files[0])).expect("Open raster file error.");
    let first_metas = RasterMetadata::from(&first_ds);
    let first_res = first_metas.res;
    let nodataval = first_metas.nodata;
    let dt = first_metas.dtype;
    let mut out_count = 1;
    match output_count {
        Some(c) => out_count = c,
        None => out_count = first_metas.count,
    }

    let mut band_index: Vec<isize> = Vec::new(); 
    match output_band_index {
        Some(idx) => out_count = idx.len() as isize,
        None => band_index = (1..=out_count).collect::<Vec<isize>>(),
    }
    println!("{:?}", band_index);

    let [mut dst_w, mut dst_s, mut dst_e, mut dst_n] = [0.0; 4];
    match output_bounds {
        Some(b) => {
            dst_w = b[0];
            dst_s = b[1];
            dst_e = b[2];
            dst_n = b[3];
        }
        None => {
            let b = first_metas.bounds;
            let mut xs: Vec<f64> = Vec::with_capacity(files.len() * 2);
            let mut ys: Vec<f64> = Vec::with_capacity(files.len() * 2);
            xs.push(b[0]); xs.push(b[2]);
            ys.push(b[1]); ys.push(b[3]);
            for f in &files[1..] {
                let ds = Dataset::open(&Path::new(f)).expect("Open raster file error.");
                let gt = ds.geo_transform().unwrap();
                
                let rs = ds.raster_size();
                let b = raster_boundary(&gt, &rs);
                xs.push(b[0]); xs.push(b[2]);
                ys.push(b[1]); ys.push(b[3]);
            }
            
            xs.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            println!("{:?}", xs);
            ys.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            dst_w = *xs.first().unwrap();
            dst_s = *ys.first().unwrap();
            dst_e = *xs.last().unwrap();
            dst_n = *ys.last().unwrap();
        }
    }

    let mut res = (0.0, 0.0);
    match output_res {
        Some(r) => res = r,
        None => res = first_res,
    }

    let output_width = ((dst_e - dst_w) / res.0).round() as isize;
    let output_height = ((dst_n - dst_s) / res.1).round() as isize;
    let output_transform = [dst_w, res.0, 0.0, dst_n, 0.0, -res.1];
    println!("{:?}", output_transform);


    let mut nodata: Option<f64> = None;
    match output_nodata {
        Some(n) => nodata = Some(n),
        None => {
            match nodataval {
                Some(n) => nodata = Some(n),
                None => nodata = None,
            }
        }
    }

    let dtype = first_metas.dtype;
    
    type T = u8;
    if dtype == 1 {
        type T = u8;
    }else if dtype == 2 {
        type T = u16;
    }
    // dst driver
    let driver = Driver::get("GTiff").unwrap();
    // let mut output_ds = driver.create("/data/mosaic.tif", output_width, output_height, out_count).unwrap();
    let mut output_ds = driver.create_with_band_type::<T>(output_file, output_width, output_height, out_count).unwrap();

    output_ds.set_geo_transform(&output_transform).unwrap();
    output_ds.set_spatial_ref(&first_ds.spatial_ref().unwrap()).unwrap();

    
    // Read write
    for f in &files {
        let ds = Dataset::open(&Path::new(f)).expect("Open raster file error.");
        let src_gt = ds.geo_transform().unwrap();               
        let rs = ds.raster_size();
        let src_b = raster_boundary(&src_gt, &rs);
        let src_w = src_b[0]; let src_s = src_b[1];
        let src_e = src_b[2]; let src_n = src_b[3];

        let int_w = if src_w > dst_w {src_w} else {dst_w};
        let int_s = if src_s > dst_s {src_s} else {dst_s};
        let int_e = if src_e < dst_e {src_e} else {dst_e};
        let int_n = if src_n < dst_n {src_n} else {dst_n};

        let src_window = calculate_window(int_w, int_s, int_e, int_n, &src_gt);

        let dst_window = calculate_window(int_w, int_s, int_e, int_n, &output_transform);


        for i in &band_index {
            let src_band = ds.rasterband(*i).unwrap();
            // let src_data = src_band.read_band_as::<T>().unwrap();
            let src_data = src_band.read_as::<T>(
                src_window.position,
                src_window.size,
                dst_window.size,
                Some(ResampleAlg::Bilinear)
            ).unwrap();

            let mut dst_band = output_ds.rasterband(*i).unwrap();
            if let Some(n) = nodata {
                dst_band.set_no_data_value(n).unwrap();
            }
            dst_band.write(
                dst_window.position, 
                dst_window.size, 
                &src_data,
            ).unwrap();
            
        }
    }
}

struct Window {
    position: (isize, isize),
    size: (usize, usize),
}

fn calculate_window(left: f64, bottom: f64, right: f64, top: f64, geo_transform: &[f64; 6]) -> Window {

    let x = ((left - geo_transform[0]) / geo_transform[1]).round() as isize;
    let y = ((top - geo_transform[3]) / geo_transform[5]).round() as isize;
    let width = ((right - left) / geo_transform[1]).round() as usize;
    let height = ((bottom - top) / geo_transform[5]).round() as usize;
    Window {
        position: (x, y),
        size: (width, height),
    }
}