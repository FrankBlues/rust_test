use std::ffi::OsStr;
use std::path::Path;

use gdal::raster::{GdalType, ResampleAlg};
use gdal::{Dataset, Driver};
// use gdal_sys::{GDALFillRaster, GDALGetRasterBand, GDALSetRasterNoDataValue,
//     GDALRasterBandH, GDALRWFlag, GDALRasterIO, GDALClose, GDALDeleteRasterNoDataValue};
// use libc::c_int;
// extern crate log;
use log::{debug, info};
// use ndarray::{s, ArcArray, Array2, Dim, Zip};
// extern crate blas_src;

use crate::{raster_boundary, RasterMetadata, calculate_window};

/// Copy valid pixels from input files to an output file.
/// All files must have the same number of bands, data type, and
/// coordinate reference system.
/// Input files are merged in their listed order.
///
/// # Arguments
/// * files - files to mosaic
/// * output_file - result raster file
/// * output_res - x, y cell size
/// * output_nodata - nodata value, if None, same as input.
/// * output_count - band count, if None, same as input.
/// * output_band_index - if None, range 1 to count.
///
/// # Examples
/// '''
/// let files = vec!["/data/test_mosaic/s_16_xian_prj3.tif", "/data/test_mosaic/s_16_xian_prj4.tif", "/data/test_mosaic/s_16_xian_prj7.tif"];
/// let output_file = "/data/mosaic8.tif";
/// let output_res: Option<(f64, f64)> = None;
/// let output_nodata: Option<f64> = Some(0.0);
/// let output_count: Option<isize> = None;
/// let output_bounds: Option<[f64; 4]> = None;
/// let output_band_index: Option<Vec<isize>> = None;
/// let resample_method = ResampleAlg::Bilinear;
///
/// merge(files, output_file, output_res, output_nodata, output_count, output_bounds, output_band_index);
/// '''
///
pub fn merge<T: AsRef<OsStr>>(
    files: Vec<T>,
    output_file: &str,
    output_res: Option<(f64, f64)>,
    output_nodata: Option<f64>,
    output_count: Option<isize>,
    output_bounds: Option<[f64; 4]>,
    output_band_index: Option<Vec<isize>>,
    resample_method: ResampleAlg,
) -> Result<RasterMetadata, Box<dyn std::error::Error>> {
    info!("Total {:?} files to merge.", files.len());

    let out_metas = output_merge_metas(
        &files,
        output_res,
        output_nodata,
        output_count,
        output_bounds,
    )?;
    let dtype = out_metas.dtype;
    match dtype {
        1 => {
            type U = u8; //  GDALDataType::GDT_Byte
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        2 => {
            type U = u16; // GDALDataType::GDT_UInt16
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        3 => {
            type U = i16; // GDALDataType::GDT_Int16
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        4 => {
            type U = u32; // GDALDataType::GDT_UInt32
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        5 => {
            type U = i32; // GDALDataType::GDT_Int32
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        6 => {
            type U = f32; // GDALDataType::GDT_Float32
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        7 => {
            type U = f64; // GDALDataType::GDT_Float64
            return merge_files::<_, U>(
                files,
                output_file,
                &out_metas,
                resample_method,
                output_band_index,
            );
        }
        _ => panic!("Not matched type."),
    }
}

/// Generate output raster metadata.
pub fn output_merge_metas<T: AsRef<OsStr>>(
    files: &Vec<T>,
    output_res: Option<(f64, f64)>,
    output_nodata: Option<f64>,
    output_count: Option<isize>,
    output_bounds: Option<[f64; 4]>,
) -> Result<RasterMetadata, Box<dyn std::error::Error>> {
    // Dataset
    info!("Read the first file to get needed infos.");
    let first_ds = Dataset::open(&Path::new(&files[0])).expect("Open raster file error.");
    let first_metas = RasterMetadata::from(&first_ds);
    let mut dst_meta = first_metas.clone();
    let first_res = first_metas.res;
    let nodataval = first_metas.nodata;
    let out_count;
    match output_count {
        Some(c) => out_count = c,
        None => out_count = first_metas.count,
    }
    info!("Output band count: {}", out_count);
    dst_meta.count = out_count;

    let dst_w;
    let dst_s;
    let dst_e;
    let dst_n;
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
            xs.push(b[0]);
            xs.push(b[2]);
            ys.push(b[1]);
            ys.push(b[3]);
            for f in &files[1..] {
                let ds = Dataset::open(&Path::new(f)).expect("Open raster file error.");
                let gt = ds.geo_transform().unwrap();

                let rs = ds.raster_size();
                let b = raster_boundary(&gt, &rs);
                xs.push(b[0]);
                xs.push(b[2]);
                ys.push(b[1]);
                ys.push(b[3]);
            }

            xs.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            ys.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            dst_w = *xs.first().unwrap();
            dst_s = *ys.first().unwrap();
            dst_e = *xs.last().unwrap();
            dst_n = *ys.last().unwrap();
        }
    }
    dst_meta.bounds = [dst_w, dst_s, dst_e, dst_n];
    info!("Output bounds: {:?}", dst_meta.bounds);

    let res;
    match output_res {
        Some(r) => res = r,
        None => res = first_res,
    }
    dst_meta.res = res;
    info!("Output resolutions: {:?}", res);

    let output_width = ((dst_e - dst_w) / res.0).round() as isize;
    let output_height = ((dst_n - dst_s) / res.1).round() as isize;
    let output_transform = [dst_w, res.0, 0.0, dst_n, 0.0, -res.1];
    debug!("New transform: {:?}", output_transform);
    dst_meta.rows = output_height as usize;
    dst_meta.cols = output_width as usize;
    dst_meta.geo_transform = output_transform;

    let nodata;
    match output_nodata {
        Some(n) => nodata = Some(n),
        None => match nodataval {
            Some(n) => nodata = Some(n),
            None => nodata = None,
        },
    }
    dst_meta.nodata = nodata;
    info!("Output nodata value: {:?}", nodata);
    Ok(dst_meta)
}

/// Read src data, write to dst data.
pub fn merge_files<T: AsRef<OsStr>, U: Copy + GdalType + std::convert::Into<f64> + PartialEq>(
    files: Vec<T>,
    output_file: &str,
    out_metas: &RasterMetadata,
    resample_method: ResampleAlg,
    output_band_index: Option<Vec<isize>>,
) -> Result<RasterMetadata, Box<dyn std::error::Error>> {
    let mut out_count = out_metas.count;
    let band_index: Vec<isize>;
    match output_band_index {
        Some(idx) => {
            band_index = idx;
            out_count = band_index.len() as isize;
        }
        None => band_index = (1..=out_count).collect::<Vec<isize>>(),
    }
    info!("Band index: {:?}", band_index);
    let mut return_metas = out_metas.clone();
    return_metas.count = out_count;

    // dst driver same as src
    let driver_name;
    if output_file.to_lowercase().ends_with(".tif") || output_file.to_lowercase().ends_with(".tiff")
    {
        driver_name = "GTiff";
    } else if output_file.to_lowercase().ends_with(".img") {
        driver_name = "HFA";
    } else {
        driver_name = &out_metas.driver;
    }
    info!("Output driver: {}", driver_name);
    let driver = Driver::get(driver_name).unwrap();
    // let mut output_ds = driver.create("/data/mosaic.tif", output_width, output_height, out_count).unwrap();
    info!("Create output dataset");
    let mut output_ds = driver
        .create_with_band_type::<U>(
            output_file,
            out_metas.cols as isize,
            out_metas.rows as isize,
            out_count,
        )
        .unwrap();

    output_ds
        .set_geo_transform(&out_metas.geo_transform)
        .unwrap();
    if let Some(sr) = &out_metas.srs {
        output_ds.set_spatial_ref(&sr).unwrap();
    }

    // fill with nodata    
    // if let Some(n) = out_metas.nodata {
    //     let c_dataset = unsafe{output_ds.c_dataset()};
    //     for i in &band_index {
    //         unsafe {
    //             let rb = GDALGetRasterBand(c_dataset, *i as i32);
    //             GDALFillRaster(rb, n, 0.0);
    //             // GDALDeleteRasterNoDataValue(rb);
    //             // GDALSetRasterNoDataValue(rb, n);
    //         }
    //     }
    // }

    let [dst_w, dst_s, dst_e, dst_n] = out_metas.bounds;

    // Read write
    info!("Writing data..");
    for f in &files {
        debug!("Processing file: {}.", OsStr::new(f).to_str().unwrap());
        let ds = Dataset::open(&Path::new(f)).expect("Open raster file error.");
        let src_gt = ds.geo_transform().unwrap();
        let rs = ds.raster_size();
        let src_b = raster_boundary(&src_gt, &rs);
        let src_w = src_b[0];
        let src_s = src_b[1];
        let src_e = src_b[2];
        let src_n = src_b[3];

        let int_w = if src_w > dst_w { src_w } else { dst_w };
        let int_s = if src_s > dst_s { src_s } else { dst_s };
        let int_e = if src_e < dst_e { src_e } else { dst_e };
        let int_n = if src_n < dst_n { src_n } else { dst_n };

        let src_window = calculate_window(int_w, int_s, int_e, int_n, &src_gt);
        let dst_window = calculate_window(int_w, int_s, int_e, int_n, &out_metas.geo_transform);

        for (i_src, i) in (&band_index).iter().enumerate() {
            let mut src_band = ds.rasterband(*i).unwrap();
            if let Some(n) = out_metas.nodata {
                src_band.set_no_data_value(n).unwrap();
            }
            let mut src_data = src_band
                .read_as::<U>(
                    src_window.position,
                    src_window.size,
                    dst_window.size,
                    Some(resample_method),
                )
                .unwrap();
            // ndarray
            // let mut src_array = src_band.read_as_array::<U>(
            //     src_window.position, src_window.size, dst_window.size, Some(resample_method)).unwrap();

            let mut dst_band= output_ds.rasterband(i_src as isize + 1).unwrap();

            // ndarray
            // let dst_array = dst_band.read_as_array::<U>(
            //     dst_window.position, dst_window.size, dst_window.size, Some(resample_method)).unwrap();

            if let Some(n) = out_metas.nodata {
                dst_band.set_no_data_value(n).unwrap();

                // ndarray
                // Zip::from(&mut src_array).and(&dst_array).for_each(|a, &bb| {
                //     if (*a).into() == n && bb.into() != n {
                //         *a = bb;
                //     }
                // });
                // let buffer = Buffer::new(
                //     (dst_window.size.0, dst_window.size.1),
                //     src_array.into_raw_vec(),
                // );
                // dst_band
                //     .write(dst_window.position, dst_window.size, &buffer)
                //     .unwrap();

                // Vec
                let dst_data = dst_band
                    .read_as::<U>(
                        dst_window.position,
                        dst_window.size,
                        dst_window.size,
                        Some(resample_method),
                    )
                    .unwrap();
            
                for (i, d) in src_data.data.iter_mut().enumerate() {
                    if (*d).into() == n && dst_data.data[i].into() != n {
                        *d = dst_data.data[i];
                    }
                }
                dst_band
                    .write(dst_window.position, dst_window.size, &src_data)
                    .unwrap();
            } else {
                dst_band
                    .write(dst_window.position, dst_window.size, &src_data)
                    .unwrap();
            }
        }
    }
    
    info!("END");
    Ok(return_metas)
}
