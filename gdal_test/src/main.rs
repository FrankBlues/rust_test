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

mod merge;
use merge::merge;

// extern crate log;
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
    env_logger::init();

    // let raster_dir = "/data/rtree_data/images/tif";
    // let in_raster = "/data/rtree_data/images/tif/s_16_xian_prj7.tif";
    // let first_ds = Dataset::open(&Path::new(in_raster)).expect("Open raster file error.");
    // let first_metas = RasterMetadata::from(&first_ds);
    // println!("{:?}", first_metas.nodata);

    // let ds = Dataset::open(Path::new(in_raster)).unwrap();
    // reproject(&ds, 32648, "/data/test_mosaic/s_16_xian_prj7.tif");

    let output_file = "/data/mosaic113.tif";
    let output_count: Option<isize> = None;
    let output_bounds: Option<[f64; 4]> = None;
    let output_res: Option<(f64, f64)> = None;
    // let output_res: Option<(f64, f64)> = Some((3e-5, 4e-5));
    let output_nodata: Option<f64> = None;
    let output_nodata: Option<f64> = Some(0.0);
    let output_band_index: Option<Vec<isize>> = None;
    let resample_method = ResampleAlg::Bilinear;

    let mut files = vec![];
    for entry in get_files("/data/test_mosaic", ".tif").unwrap() {
        match entry {
            Ok(p) => files.push(p.to_str().unwrap().to_owned()),
            Err(e) => println!("{:?}", e),
        }
    }
    merge(files, output_file, output_res, output_nodata, output_count, output_bounds, output_band_index, resample_method);

}