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
    GDALCreateWarpOperation, GDALCreateWarpOptions, GDALWarpOperationH, GDALWarpOptions,
    GDALCreateGenImgProjTransformer, GDALCreate, GDALSuggestedWarpOutput, GDALTransformerFunc,
    GDALGenImgProjTransform, GDALDestroyGenImgProjTransformer, GDALSetProjection, GDALSetGeoTransform,
    GDALChunkAndWarpImage, CPLMalloc, GDALClose, GDALWarpInitDstNoDataReal, GDALWarpInitSrcNoDataReal,
    GDALTermProgress, GDALChunkAndWarpMulti, GDALGetCacheMax, GDALGetCacheMax64, GDALSetCacheMax64,
    GDALSetCacheMax, GDALCreateGenImgProjTransformer2, GDALCreateApproxTransformer, GDALApproxTransform,
    GDALApproxTransformerOwnsSubtransformer, CSLSetNameValue, CSLFetchNameValue

};

use std::fs::File;
use std::io::BufReader;
extern crate xml;
use std::collections::HashMap;
use xml::reader::{EventReader, XmlEvent};

use std::ffi::CString;

use std::time::SystemTime;

fn main() {
    let mut bounds: HashMap<String, f64> = HashMap::new();
    let xml_file = "/data/GF7_DLC_E129.3_N45.9_20200429_L1L0000091709-BWDMUX/GF7_DLC_E129.3_N45.9_20200429_L1L0000091709-BWDMUX.xml";


    // let in_raster = "/data/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601-PAN2.tiff";
    // let out_raster = "/data/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601/GF2_PMS2_E109.1_N34.2_20181026_L1A0003549601-PAN2.tiff";

    let in_raster = "/data/SV1-02_20170614_L1B0000085483_1109170043001_01.tiff";
    // let val_value: u8 = 158;
    // let (r, g, b) = (22 as u8, 181 as u8, 255 as u8);
    let ds = Dataset::open(Path::new(in_raster)).unwrap();
    // println!("{:?}", ds.raster_size());
    // match ds.geo_transform() {
    //     Ok(trans) => println!("{:?}", trans),
    //     Err(e) => eprintln!("Cannot get geotransform, try from xml error is {}", e),
    // }
    // println!("{:?}", ds.geo_transform());
    let src_sr = ds.spatial_ref().unwrap();
    let src_wkt = src_sr.to_wkt().unwrap();
    println!("{}", src_wkt);
    // let count = ds.raster_count();
    // if count != 1 {
    //     println!("Warning: input raster has more than 1 ({}) band", &count);
    //     println!("Try using the first band.");
    // }
    let rb = ds.rasterband(1).unwrap();
    // let (x_size, y_size) = ds.raster_size();
    // println!("{:?}, {:?}", x_size, y_size);
    let dtype = rb.band_type();
    // type T = u8;
    // let (block_x, block_y) = rb.block_size();
    // println!("{:?}", rb.block_size());
    // let (x_remain, _) = rb.actual_block_size(((x_size / block_x) as isize, 0)).unwrap();
    // let (_, y_remain) = rb.actual_block_size((0, (y_size / block_y) as isize)).unwrap();
    // let prj = ds.projection();
    // let dst_wkt = "PROJCS[\"WGS 84 / UTM zone 48N\",GEOGCS[\"WGS 84\",DATUM[\"WGS_1984\",SPHEROID[\"WGS 84\",6378137,298.257223563,AUTHORITY[\"EPSG\",\"7030\"]],AUTHORITY[\"EPSG\",\"6326\"]],PRIMEM[\"Greenwich\",0,AUTHORITY[\"EPSG\",\"8901\"]],UNIT[\"degree\",0.0174532925199433,AUTHORITY[\"EPSG\",\"9122\"]],AUTHORITY[\"EPSG\",\"4326\"]],PROJECTION[\"Transverse_Mercator\"],PARAMETER[\"latitude_of_origin\",0],PARAMETER[\"central_meridian\",105],PARAMETER[\"scale_factor\",0.9996],PARAMETER[\"false_easting\",500000],PARAMETER[\"false_northing\",0],UNIT[\"metre\",1,AUTHORITY[\"EPSG\",\"9001\"]],AXIS[\"Easting\",EAST],AXIS[\"Northing\",NORTH],AUTHORITY[\"EPSG\",\"32648\"]]";

    let driver = Driver::get("GTiff").unwrap();
    // let mut new_ds = driver.create_with_band_type::<u8>("/data/test_proj.tif", x_size as isize, y_size as isize, count).unwrap();
    let dst_sr = SpatialRef::from_epsg(4326).unwrap();
    let dst_wkt = dst_sr.to_wkt().unwrap();
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

    unsafe {
        // GDALSetCacheMax64(4000000000);
        println!("GDALGetCacheMax64: {:?}", GDALGetCacheMax64());
        // to C
        let src_ds = ds.c_dataset();
        let src_wkt_p = CString::new(src_wkt).unwrap();
        let dst_wkt_p = CString::new(dst_wkt).unwrap();
        let mut dst_trans = [0.0; 6];

        let mut n_pixels: i32 = 0;
        let mut n_lines: i32 = 0;

        let h_trans_arg = GDALCreateGenImgProjTransformer(src_ds, src_wkt_p.as_ptr(), null_mut(), dst_wkt_p.as_ptr(), 0, 0.0, 0);
        
        let warp_output = GDALSuggestedWarpOutput(src_ds, Some(GDALGenImgProjTransform), 
            h_trans_arg, dst_trans.as_mut_ptr(), &mut n_pixels, &mut n_lines);
        GDALDestroyGenImgProjTransformer(h_trans_arg);
        
        println!("{}", warp_output);
        println!("trans: {:?}, cols: {}, rows:{}", dst_trans, n_pixels, n_lines);

        let dst_file = CString::new("/data/sv1_4326.tif").unwrap();
        let dst_ds = GDALCreate(driver.c_driver(), dst_file.as_ptr(), n_pixels, n_lines, ds.raster_count() as i32, dtype, null_mut());
    
        // Write out the projection definition.
        GDALSetProjection(dst_ds, dst_wkt_p.as_ptr());
        GDALSetGeoTransform(dst_ds, dst_trans.as_mut_ptr());
        // let mut dst_ds = driver.create("/data/test_prjj.tif", n_pixels as isize, n_lines as isize, ds.raster_count()).unwrap();
        // dst_ds.set_geo_transform(&dst_trans).unwrap();
        // dst_ds.set_spatial_ref(&dst_sr).unwrap();
        // dst_ds.set_projection(&dst_wkt).unwrap();

        let ops = GDALCreateWarpOptions();
        
        // (*ops).dfWarpMemoryLimit = 4000000000.0;
        (*ops).hSrcDS = src_ds;
        (*ops).hDstDS = dst_ds;
        (*ops).nBandCount = ds.raster_count() as i32;
        // use std::mem::size_of;
        // let t = CPLMalloc(size_of::<i32>() * ((*ops).nBandCount as usize));
        // (*ops).panSrcBands = t;
        let mut pan_src = 1_i32;
        let mut pan_dst = 1_i32;
        let mut nodata = -99999.0;
        let err_torlerance = 0.125;
        (*ops).panSrcBands = &mut pan_src;
        (*ops).panDstBands = &mut pan_dst;

        (*ops).pfnProgress = Some(GDALTermProgress);

        if err_torlerance == 0.0 {
            (*ops).pTransformerArg = GDALCreateGenImgProjTransformer2( src_ds,
                dst_ds, null_mut() );
    
            // (*ops).pTransformerArg = GDALCreateGenImgProjTransformer( src_ds,
            //                          src_wkt_p.as_ptr(),
            //                          dst_ds,
            //                          dst_wkt_p.as_ptr(),
            //                          0, 0.0, 0 );
            (*ops).pfnTransformer = Some(GDALGenImgProjTransform);
        } else {  // Use an approximated transformation instead of exact reprojection for each pixel to be transformed
            let mut hTransformArg = GDALCreateGenImgProjTransformer2( src_ds,
                dst_ds, null_mut() );
            hTransformArg =
                GDALCreateApproxTransformer( Some(GDALGenImgProjTransform),
                hTransformArg, 0.125 );
            GDALApproxTransformerOwnsSubtransformer(hTransformArg, 1);
            (*ops).pTransformerArg = hTransformArg;
            (*ops).pfnTransformer = Some(GDALApproxTransform);
        }

        (*ops).padfSrcNoDataReal = &mut nodata;
        (*ops).padfSrcNoDataImag =&mut 0.0_f64;
        (*ops).padfDstNoDataReal =&mut nodata;
        // GDALWarpInitDstNoDataReal(ops, 0.0);
        // GDALWarpInitSrcNoDataReal(ops, 0.0);
        (*ops).padfDstNoDataImag =&mut 0.0_f64;
        let _init_dest = CString::new("INIT_DEST").unwrap();
        let _no_data = CString::new("NO_DATA").unwrap();
        let _n_threads = CString::new("NUM_THREADS").unwrap();
        let _all_cpus = CString::new("ALL_CPUS").unwrap();
        // println!("{:?}", CSLFetchNameValue((*ops).papszWarpOptions, _init_dest.as_ptr()));
        if (*ops).padfDstNoDataReal != null_mut() {
            (*ops).papszWarpOptions = CSLSetNameValue((*ops).papszWarpOptions, _init_dest.as_ptr(), _no_data.as_ptr());
        }
        // println!("{:?}", *CSLFetchNameValue((*ops).papszWarpOptions, _init_dest.as_ptr()));
        (*ops).papszWarpOptions = CSLSetNameValue((*ops).papszWarpOptions, _n_threads.as_ptr(), _all_cpus.as_ptr());
        
        // println!("{:?}", *ops);

        let st_time = SystemTime::now();

        let o_operation = GDALCreateWarpOperation(ops);
        // GDALChunkAndWarpImage(o_operation, 0, 0, n_pixels, n_lines);
        GDALChunkAndWarpMulti(o_operation, 0, 0, n_pixels, n_lines);

        // GDALClose( src_ds );
        GDALClose( dst_ds );

        let lt_time = SystemTime::now();
        println!(
            "spend {:?}",
            SystemTime::duration_since(&lt_time, st_time).unwrap()
        );

    }

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
