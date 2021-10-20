use gdal::spatial_ref::SpatialRef;
use gdal::{Dataset, Driver};
use gdal_sys::{
    CSLSetNameValue, GDALApproxTransform, GDALApproxTransformerOwnsSubtransformer,
    GDALChunkAndWarpMulti, GDALClose, GDALCreate, GDALCreateApproxTransformer,
    GDALCreateGenImgProjTransformer, GDALCreateGenImgProjTransformer2, GDALCreateWarpOperation,
    GDALCreateWarpOptions, GDALDestroyGenImgProjTransformer, GDALGenImgProjTransform,
    GDALSetGeoTransform, GDALSetProjection, GDALSuggestedWarpOutput, GDALTermProgress,
};

use gdal::errors::*;
use gdal_sys::{CPLErr, GDALResampleAlg};
use std::ffi::CStr;
use std::ffi::CString;
use std::ptr::{null, null_mut};

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
