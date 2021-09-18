use gdal::spatial_ref::SpatialRef;
use gdal::spatial_ref::CoordTransform;
use gdal_sys::{GDALReprojectImage, GDALCreateAndReprojectImage};

use std::ffi::CString;

/// 
fn reproject_to_file(src: &Dataset, dst: &str, dst_prj: &str) -> Result<()> {
    let rv = unsafe {
        gdal_sys::GDALCreateAndReprojectImage(
            src.c_dataset(),
            null(),
            CString::new(dst).unwrap().as_ptr(),
            CString::new(dst_prj).unwrap().as_ptr(),
            null_mut(),
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

fn reproject() {
    // calculate_default_transform
    // prject
        // GDALWarpOptions 
        // GDALWarpOperation 
}
