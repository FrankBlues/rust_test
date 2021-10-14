use gdal::raster::{Buffer, GdalType, RasterBand};
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

fn main() {
    let in_raster = "/data/rtree_data/images/win_161.tif";
    // let in_raster = "/data/S2A_MSIL1C_20210901T025551_N0301_R032_T50SLH_20210901T050029_13.jpg";
    let ds = Dataset::open(Path::new(in_raster)).unwrap();
    reproject(&ds, 4326, "/data/proj_result.tif");
}
