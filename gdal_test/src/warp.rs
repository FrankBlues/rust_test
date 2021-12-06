use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::{null, null_mut};
use std::path::Path;
use std::collections::HashMap;
use std::convert::TryInto;

use gdal::spatial_ref::SpatialRef;
use gdal::{Dataset, Driver, Metadata};
use gdal_sys::{
    CSLSetNameValue, GDALApproxTransform, GDALApproxTransformerOwnsSubtransformer,
    GDALChunkAndWarpMulti, GDALClose, GDALCreate, GDALCreateApproxTransformer,
    GDALCreateGenImgProjTransformer, GDALCreateGenImgProjTransformer2, GDALCreateWarpOperation,
    GDALCreateWarpOptions, GDALDestroyGenImgProjTransformer, GDALGenImgProjTransform,
    GDALSetGeoTransform, GDALSetProjection, GDALSuggestedWarpOutput, GDALTermProgress,
    GDALCreateRPCTransformerV2, GDALRPCTransform, GDALRPCInfoV2
};

use gdal::errors::*;
use gdal_sys::{CPLErr, GDALResampleAlg};


use libc::c_char;

pub mod raster_projection {
    use super::*;

    /// Reproject an image and create the target reprojected image(GDALCreateAndReprojectImage).
    pub fn reproject_to_file(src: &Dataset, dst: &str, dst_prj: &str) -> Result<()> {
        let c_dst = CString::new(dst).unwrap();
        let c_dst_prj = CString::new(dst_prj).unwrap();
        let rv = unsafe {
            gdal_sys::GDALCreateAndReprojectImage(
                src.c_dataset(),               // GDALDatasetH
                null(), // the source projection. If NULL the source projection is read from from src.
                c_dst.as_ptr(), // the destination image file.
                c_dst_prj.as_ptr(), // the destination projection in wkt format.
                null_mut(), // GDALDriverH
                null_mut(), // char **papszCreateOptions
                GDALResampleAlg::GRA_Bilinear, // the type of resampling to use
                0.0,    // dfWarpMemoryLimit
                0.0, // maximum error measured in input pixels that is allowed in approximating the transformation
                None, // a GDALProgressFunc()
                null_mut(), // argument to be passed to pfnProgress. May be NULL.
                null_mut(), // warp options, normally NULL
            )
        };
        if rv != CPLErr::CE_None {
            return Err(_last_cpl_err(rv));
        }
        Ok(())
    }

    pub fn reproject(
        ds: &Dataset,
        dst_epsg_code: u32,
        out_file: &str,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        // src spacial referrence
        let src_sr = ds.spatial_ref().unwrap();
        let src_wkt = src_sr.to_wkt().unwrap();

        // raster count
        let rc = ds.raster_count() as i32;

        // dst file
        let dst_file = CString::new(out_file).unwrap();
        // dst data type
        let rb = ds.rasterband(1).unwrap();
        let dtype = rb.band_type();

        // dst driver
        let driver = Driver::get("GTiff").unwrap();

        // dst spacial referrence
        let dst_sr = SpatialRef::from_epsg(dst_epsg_code).unwrap();
        let dst_wkt = dst_sr.to_wkt().unwrap();

        // to c str
        let src_wkt_p = CString::new(src_wkt).unwrap();
        let dst_wkt_p = CString::new(dst_wkt).unwrap();
        //dst transform, cols, rows variables
        let mut dst_trans = [0.0; 6];
        let mut n_pixels: i32 = 0;
        let mut n_lines: i32 = 0;

        // warp options parameters
        let mut pan_src = 1;
        let mut pan_dst = 1;
        let mut nodata = -0.0;
        let err_torlerance = 0.125;

        unsafe {
            // Calculate dst transform, cols, rows //
            let src_ds = ds.c_dataset();
            let h_trans_arg = GDALCreateGenImgProjTransformer(
                src_ds,
                src_wkt_p.as_ptr(),
                null_mut(),
                dst_wkt_p.as_ptr(),
                0,
                0.0,
                0,
            );

            GDALSuggestedWarpOutput(
                src_ds,
                Some(GDALGenImgProjTransform),
                h_trans_arg,
                dst_trans.as_mut_ptr(),
                &mut n_pixels,
                &mut n_lines,
            );
            GDALDestroyGenImgProjTransformer(h_trans_arg);

            // Create dst dataset //
            let dst_ds = GDALCreate(
                driver.c_driver(),
                dst_file.as_ptr(),
                n_pixels,
                n_lines,
                rc,
                dtype,
                null_mut(),
            );

            // Set metadata
            GDALSetProjection(dst_ds, dst_wkt_p.as_ptr());
            GDALSetGeoTransform(dst_ds, dst_trans.as_mut_ptr());

            // Create WarpOptions //
            let ops = GDALCreateWarpOptions();

            // (*ops).dfWarpMemoryLimit = 4000000000.0;
            (*ops).hSrcDS = src_ds;
            (*ops).hDstDS = dst_ds;
            (*ops).nBandCount = 0; // all bands

            (*ops).panSrcBands = &mut pan_src; // CPLMalloc(
            (*ops).panDstBands = &mut pan_dst;

            let pan_src = &mut (1..=rc).collect::<Vec<i32>>();
            let pan_dst = &mut (1..=rc).collect::<Vec<i32>>();
            (*ops).panSrcBands = pan_src.as_mut_ptr(); // CPLMalloc(
            (*ops).panDstBands = pan_dst.as_mut_ptr();

            (*ops).pfnProgress = Some(GDALTermProgress);

            if err_torlerance == 0.0 {
                (*ops).pTransformerArg =
                    GDALCreateGenImgProjTransformer2(src_ds, dst_ds, null_mut());

                // (*ops).pTransformerArg = GDALCreateGenImgProjTransformer( src_ds,
                //                          src_wkt_p.as_ptr(),
                //                          dst_ds,
                //                          dst_wkt_p.as_ptr(),
                //                          0, 0.0, 0 );
                (*ops).pfnTransformer = Some(GDALGenImgProjTransform);
            } else {
                // Use an approximated transformation instead of exact reprojection for each pixel to be transformed
                let mut hTransformArg =
                    GDALCreateGenImgProjTransformer2(src_ds, dst_ds, null_mut());
                hTransformArg = GDALCreateApproxTransformer(
                    Some(GDALGenImgProjTransform),
                    hTransformArg,
                    0.125,
                );
                GDALApproxTransformerOwnsSubtransformer(hTransformArg, 1);
                (*ops).pTransformerArg = hTransformArg;
                (*ops).pfnTransformer = Some(GDALApproxTransform);
            }

            (*ops).padfSrcNoDataReal = &mut nodata;
            (*ops).padfSrcNoDataImag = &mut 0.0_f64;
            (*ops).padfDstNoDataReal = &mut nodata;
            (*ops).padfDstNoDataImag = &mut 0.0_f64;
            let _init_dest = CString::new("INIT_DEST").unwrap();
            let _no_data = CString::new("NO_DATA").unwrap();
            let _n_threads = CString::new("NUM_THREADS").unwrap();
            let _all_cpus = CString::new("ALL_CPUS").unwrap();
            let _u_src_nodata = CString::new("UNIFIED_SRC_NODATA").unwrap();
            let _yes = CString::new("YES").unwrap();

            let mut warp_extras = (*ops).papszWarpOptions;

            if (*ops).padfDstNoDataReal != null_mut() {
                warp_extras = CSLSetNameValue(warp_extras, _init_dest.as_ptr(), _no_data.as_ptr());
            }

            warp_extras = CSLSetNameValue(warp_extras, _n_threads.as_ptr(), _all_cpus.as_ptr());

            warp_extras = CSLSetNameValue(warp_extras, _u_src_nodata.as_ptr(), _yes.as_ptr());

            // let _n = CSLFetchNameValue(warp_extras, _u_src_nodata.as_ptr());

            // let c_str = unsafe { CStr::from_ptr(*warp_extras) };
            // println!("{}", c_str.to_string_lossy().into_owned());

            (*ops).papszWarpOptions = warp_extras;

            // Operation //
            let o_operation = GDALCreateWarpOperation(ops);
            // GDALChunkAndWarpImage(o_operation, 0, 0, n_pixels, n_lines);
            GDALChunkAndWarpMulti(o_operation, 0, 0, n_pixels, n_lines);

            // GDALClose( src_ds );
            GDALClose(dst_ds);
        }
        Ok(())
    }
}

/// Transform pixel coordinate to geo coordinates
pub fn rpc_transform_pixel(input_data: String, 
    x:&mut Vec<f64>, 
    y:&mut Vec<f64>, 
    z:&mut Vec<f64>) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    // RPC
    let dataset = Dataset::open(Path::new(&input_data)).expect("Open input image fail!!");
    let rpc = dataset.metadata_domain("RPC");

    let gdal_rpc_info;
    match rpc {
        Some(r) => gdal_rpc_info = parse_rpc_meta(r),
        None => panic!("Input image has not rpc file!!")
    }

    //  let gdal_rpc_info = parse_rpc_meta(rpc.unwrap());

    let (_x, _y, _z) = (x.as_mut_slice(), y.as_mut_slice(), z.as_mut_slice());
    unsafe{
        let rpc_transform = GDALCreateRPCTransformerV2(&gdal_rpc_info, 0, 0.0, null_mut());
        GDALRPCTransform(rpc_transform, 0, 2,
            _x.as_mut_ptr(),
            _y.as_mut_ptr(),
            _z.as_mut_ptr(),
            &mut 0);
    }
    return (x.to_owned(), y.to_owned(), z.to_owned())
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
