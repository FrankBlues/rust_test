use gdal::spatial_ref::SpatialRef;
use gdal::spatial_ref::CoordTransform;
use gdal_sys::{GDALReprojectImage, GDALCreateAndReprojectImage};

use std::ffi::CString;
use std::ptr::{null, null_mut};

/// Reproject an image and create the target reprojected image(GDALCreateAndReprojectImage).
pub fn reproject_to_file(src: &Dataset, dst: &str, dst_prj: &str) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALCreateAndReprojectImage(
            src.c_dataset(),  // GDALDatasetH
            null(),  // the source projection. If NULL the source projection is read from from src.
            CString::new(dst).unwrap().as_ptr(),  // the destination image file.
            CString::new(dst_prj).unwrap().as_ptr(), // the destination projection in wkt format.
            null_mut(),  // GDALDriverH
            null_mut(),  // char **papszCreateOptions
            GDALResampleAlg::GRA_Bilinear,  // the type of resampling to use
            0.0,  // dfWarpMemoryLimit
            0.0,  // maximum error measured in input pixels that is allowed in approximating the transformation
            None,  // a GDALProgressFunc() 
            null_mut(),  // argument to be passed to pfnProgress. May be NULL.
            null_mut(),  // warp options, normally NULL
        )
    };
    if rv != CPLErr::CE_None {
        return Err(_last_cpl_err(rv));
    }
    Ok(())
}



fn reproject() {
    // calculate_default_transform
    // prject
        // GDALWarpOptions 
        // GDALWarpOperation 
}
