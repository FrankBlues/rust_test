use gdal::raster::{Buffer, GdalType, RasterBand, ResampleAlg};
// use gdal::Metadata;
use gdal::{Dataset, Driver, LayerOptions, Metadata};
use gdal_test::{gray2rgb, write_block_thread};
use ndarray::{s, ArcArray, Array2, Dim, Zip};
use std::path::Path;

extern crate threadpool;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

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
    GDALGetMetadata,
    CPLMalloc, CSLFetchNameValue, CSLSetNameValue, GDALApproxTransform,
    GDALApproxTransformerOwnsSubtransformer, GDALChunkAndWarpImage, GDALChunkAndWarpMulti,
    GDALClose, GDALCreate, GDALCreateApproxTransformer, GDALCreateGenImgProjTransformer,
    GDALCreateGenImgProjTransformer2, GDALCreateWarpOperation, GDALCreateWarpOptions,
    GDALDestroyGenImgProjTransformer, GDALGenImgProjTransform, GDALGetCacheMax, GDALGetCacheMax64,
    GDALSetCacheMax, GDALSetCacheMax64, GDALSetGeoTransform, GDALSetProjection,
    GDALSuggestedWarpOutput, GDALTermProgress, GDALTransformerFunc, GDALWarpInitDstNoDataReal,
    GDALWarpInitSrcNoDataReal, GDALWarpOperationH, GDALWarpOptions, GDALRasterBandH,
    GDALGetRasterBand, GDALPolygonize, OGRwkbGeometryType, 
};
use gdal_sys::{GDALRPCInfoV2, GDALExtractRPCInfoV2, GDALCreateRPCTransformerV2, GDALRPCTransform};
use gdal_sys::{GDALOpen};

use std::fs::File;
use std::io::BufReader;
extern crate xml;
use xml::reader::{EventReader, XmlEvent};

use std::ffi::CString;

use std::time::SystemTime;

// use warp::raster_projection::reproject;


use gdal_test::get_files;

use gdal_test::{raster_boundary, RasterMetadata, guess_driver_by_name};
use std::any::type_name;
use std::any::Any;
use std::convert::TryInto;

use gdal::vector::OGRFieldType;

use uuid::Uuid;


// use gdal_test::merge;

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

fn str2cstr(s: &str) -> CString {
    CString::new(s).unwrap()
}

fn string2mut_cchar(s: String) -> *mut c_char {
    let mut bytes = s.into_bytes();
    bytes.push(b'\0');
    let mut cchar: Vec<c_char> = bytes.iter().map(|w| *w as c_char).collect();
    cchar.as_mut_slice().as_mut_ptr()
}

fn parse_rpc_meta(rpc_meta: Vec<String>) -> GDALRPCInfoV2 {
    let mut rpcs = HashMap::new();
    for s in &rpc_meta {
        let elements: Vec<&str> = s.split("=").collect();
        rpcs.insert(String::from(elements[0]), String::from(elements[1]));
    }
    GDALRPCInfoV2 {
        dfLINE_OFF: parse_float(rpcs.get("LINE_OFF").unwrap()),
        dfSAMP_OFF: parse_float(rpcs.get("SAMP_OFF").unwrap()),
        dfLAT_OFF: parse_float(rpcs.get("LAT_OFF").unwrap()),
        dfLONG_OFF: parse_float(rpcs.get("LONG_OFF").unwrap()),
        dfHEIGHT_OFF: parse_float(rpcs.get("HEIGHT_OFF").unwrap()),
        dfLINE_SCALE: parse_float(rpcs.get("LINE_SCALE").unwrap()),
        dfSAMP_SCALE: parse_float(rpcs.get("SAMP_SCALE").unwrap()),
        dfLAT_SCALE: parse_float(rpcs.get("LAT_SCALE").unwrap()),
        dfLONG_SCALE: parse_float(rpcs.get("LONG_SCALE").unwrap()),
        dfHEIGHT_SCALE: parse_float(rpcs.get("HEIGHT_SCALE").unwrap()),
        adfLINE_NUM_COEFF: parse_coef(rpcs.get("LINE_NUM_COEFF").unwrap()),
        adfLINE_DEN_COEFF: parse_coef(rpcs.get("LINE_DEN_COEFF").unwrap()),
        adfSAMP_NUM_COEFF: parse_coef(rpcs.get("SAMP_NUM_COEFF").unwrap()),
        adfSAMP_DEN_COEFF: parse_coef(rpcs.get("SAMP_DEN_COEFF").unwrap()),
        dfMIN_LONG: -180.0,
        dfMIN_LAT: -90.0,
        dfMAX_LONG: 180.0,
        dfMAX_LAT: 90.0,
        dfERR_BIAS: parse_float(rpcs.get("ERR_BIAS").unwrap()),
        dfERR_RAND: parse_float(rpcs.get("ERR_RAND").unwrap()),
    }
}

fn parse_float(s: &String) -> f64 {
    s.parse::<f64>().unwrap()
}

fn parse_coef(s: &String) -> [f64; 20] {
    let coefs = s.split(" ")
        .map(|e| parse_float(&String::from(e)))
        .collect::<Vec::<f64>>();
    assert_eq!(coefs.len(), 20);
    coefs.as_slice().try_into().expect("slice with incorrect length")
}

fn main() {
    // RPC
    let input_data = "/data/GF2_PMS1_E108.9_N34.2_20181026_L1A0003549596/GF2_PMS1_E108.9_N34.2_20181026_L1A0003549596-MSS1.tiff";
    let dataset = Dataset::open(Path::new(input_data)).unwrap();
    let (width, height) = dataset.raster_size();
    let rpc = dataset.metadata_domain("RPC");

    let gdal_rpc_info = parse_rpc_meta(rpc.unwrap());

    unsafe{

        let rpc_transform = GDALCreateRPCTransformerV2(&gdal_rpc_info, 0, 0.0, null_mut());
        let mut x = [0.0, 0.0];
        let mut y = [0.0, height as f64];
        let mut z = [0.0, 0.0];
        GDALRPCTransform(rpc_transform, 0, 2, x.as_mut_ptr(), y.as_mut_ptr(), z.as_mut_ptr(), &mut 0);
        println!("{:?}, {:?}", x, y)
    }


    // Polygonize
    // let uid = Uuid::new_v4();
    // let mut buf = [b'!'; 36];
    // let ds_name = uid.to_simple().encode_lower(&mut buf);
    // println!("uuid:{}", ds_name);

    // println!("Hello world");
    // // env_logger::init();
    // let input_data = "/data/GF2_PMS1_E108.9_N34.2_20181026_L1A0003549596-MSS11.tif";
    // let nodata_val = 0.0;

    // let output_vector = "/data/mask.json";
    // let driver_name: Option<&str> = None;

    // type U = u8;
    // let _nodata = nodata_val as U;
    // let dataset = Dataset::open(Path::new(input_data)).unwrap();
    // let metas = RasterMetadata::from(&dataset);
    // let rb = dataset.rasterband(1).unwrap();
    // let buffer = rb.read_band_as::<U>().unwrap();

    // // mask band in memory
    // println!("Create in memery dataset.");
    // let mask_driver = Driver::get("MEM").unwrap();
    // let mut mask_ds = mask_driver.create_with_band_type::<u8>(ds_name, 
    //     metas.cols as isize, metas.rows as isize, 1).unwrap();
    // mask_ds.set_geo_transform(&metas.geo_transform).unwrap();
    // if let Some(srs) = metas.srs {
    //     mask_ds.set_spatial_ref(&srs).unwrap();
    // }
    // let mut mask_band = mask_ds.rasterband(1).unwrap();
    // let mut mask_data = mask_band.read_band_as::<u8>().unwrap();

    // // println!("sum of mask data before: {}", sum(&mask_data.data));
    // for (m, src) in mask_data.data.iter_mut().zip(buffer.data.iter()) {
    //     if *src != _nodata {
    //         *m = 1;
    //     } else {
    //         if _nodata != 0 {
    //             *m = 0;
    //         }
    //     }
    // }
    
    // // println!("sum of mask data after: {}", sum(&mask_data.data));
    // mask_band.write((0, 0), (metas.cols, metas.rows), &mask_data).unwrap();
    // // buffer.data.as_ptr
    // println!("create shapefile");

    // let vector_driver;

    // match driver_name {
    //     Some(drv) => vector_driver = drv,
    //     None => {
    //         match guess_driver_by_name(output_vector) {
    //             Some(drv) => vector_driver = drv,
    //             None => panic!("Cannot infer vector driver."),
    //         }
    //     }
    // }

    // let driver = Driver::get(vector_driver).unwrap();  // ESRI Shapefile
    // let mut ds = driver.create_vector_only(output_vector).unwrap();
    // let mut mask_lyr = ds.create_layer(LayerOptions {
    //     name: "mask",
    //     srs: Some(&dataset.spatial_ref().unwrap()),
    //     ty: gdal_sys::OGRwkbGeometryType::wkbPolygon,
    //     ..Default::default()
    // }).unwrap();
    // mask_lyr.create_defn_fields(&[("temp", OGRFieldType::OFTInteger)]).unwrap();
    // // mask_lyr.create_feature_fields(geometry: Geometry, field_names: &[&str], values: &[FieldValue])
    // // mask_lyr.create_feature_fields(geometry: Geometry, field_names: &[&str], values: &[FieldValue])

    // // let mask_lyr = null_mut();
    // let _connected = CString::new("8CONNECTED").unwrap();
    // let _eight = CString::new("8").unwrap();
    // unsafe {
    //     let mut options: *mut *mut c_char = null_mut();
    //     // options = CSLSetNameValue(options, _connected.as_ptr(), _eight.as_ptr());
    //     // let c_dataset = dataset.c_dataset();
    //     // let rb = GDALGetRasterBand(c_dataset, 1);
    //     let rb_mask = GDALGetRasterBand(mask_ds.c_dataset(), 1);
        
    //     GDALPolygonize(rb_mask, rb_mask, 
    //         mask_lyr.c_layer(), 0, options, None, null_mut());
    // }

    // println!("feature count {:?}", mask_lyr.feature_count());
    // for fea in mask_lyr.features() {
    //     println!("{:?}", fea.geometry().wkt());
    // }
}

fn sum(vec: &Vec<u8>) -> f64{
    let mut s = 0.0;
    for i in vec {
        s += *i as f64;
    }
    s
}
