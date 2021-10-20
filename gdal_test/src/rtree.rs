use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::Path;

use gdal::Dataset;
use glob::Paths;
use rstar::{RTree, RTreeObject, AABB};

extern crate xml;
use xml::reader::{EventReader, XmlEvent};

use crate::io_utils::{check_parent_dir, get_files};
use crate::raster_boundary;

/// image boundaries with filenames.
pub struct ImageBoundary {
    pub path: String,
    left: f64,
    bottom: f64,
    right: f64,
    top: f64,
}

/// imple RTreeObject trait to allow rtree index.
impl RTreeObject for ImageBoundary {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        AABB::from_corners([self.left, self.bottom], [self.right, self.top])
    }
}

impl ImageBoundary {
    pub fn new(path: String, left: f64, bottom: f64, right: f64, top: f64) -> ImageBoundary {
        ImageBoundary {
            path: path,
            left: left,
            bottom: bottom,
            right: right,
            top: top,
        }
    }

    /// Construct a ImageBoundary object from input raster.
    pub fn construct_from_raster(img: &str) -> Result<ImageBoundary, Box<dyn std::error::Error>> {
        let path = Path::new(img);
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let dir_name = path.parent().unwrap();
        let extension = path.extension().unwrap();
        let ds = Dataset::open(&path).expect("Open ref dataset error");
        let shape = ds.raster_size();
        match ds.geo_transform() {
            Ok(trans) => {
                let boundary = raster_boundary(&trans, &shape);
                // println!("image boundary: {:?}", boundary);
                return Ok(ImageBoundary::new(
                    String::from(img),
                    boundary[0],
                    boundary[1],
                    boundary[2],
                    boundary[3],
                ));
            }
            Err(e) => {
                println!(
                    "Cannot get geotransform, error: {}\ntry get image boundary from xml file.",
                    e
                );
                // let extension = img.split(".").collect::<Vec<&str>>().last().unwrap();
                let mut xml_file = img.replace(extension.to_str().unwrap(), "xml");
                // if "GF6_WFV"
                if file_name.starts_with("GF6_WFV") {
                    xml_file = String::from(
                        dir_name
                            .join(file_name.split("-").collect::<Vec<&str>>().first().unwrap())
                            .to_str()
                            .unwrap()
                            .to_owned()
                            + ".xml",
                    );
                }
                let boundary = parse_boundaries_from_xml(&xml_file);
                // println!("image boundary: {:?}", boundary);
                return Ok(ImageBoundary::new(
                    String::from(img),
                    boundary[0],
                    boundary[1],
                    boundary[2],
                    boundary[3],
                ));
            }
        }
    }
}

/// Parse boundaries from xml file.
fn parse_boundaries_from_xml(xml_file: &str) -> [f64; 4] {
    let path = Path::new(xml_file);
    let file = File::open(&path).expect("Open xml file error.");
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let file_name = path.file_name().unwrap().to_str().unwrap();

    let left;
    let bottom;
    let right;
    let top;

    if file_name.starts_with("GF7") {
        let elements = vec![
            "LeftTopPoint",
            "RightTopPoint",
            "RightBottomPoint",
            "LeftBottomPoint",
        ];
        let mut labels: Vec<String> = Vec::with_capacity(4);
        let mut lons: Vec<f64> = Vec::with_capacity(4);
        let mut lats: Vec<f64> = Vec::with_capacity(4);

        let mut flag = 5;

        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    flag += 1;

                    if (&elements).contains(&name.local_name.as_str()) {
                        flag = 0;
                        labels.push(name.local_name);
                    }
                }
                Ok(XmlEvent::Characters(chars)) => {
                    if flag == 3 {
                        // Longtitude
                        lons.push(chars.parse::<f64>().unwrap());
                    } else if flag == 4 {
                        // Latitude
                        lats.push(chars.parse::<f64>().unwrap());
                    } else {
                        continue;
                    }
                    if lats.len() == 4 {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
            // break;
        }
        let map: HashMap<String, (f64, f64)> = labels
            .into_iter()
            .zip(lons.into_iter().zip(lats.into_iter()))
            .collect();
        left = map
            .get("LeftTopPoint")
            .unwrap()
            .0
            .min(map.get("LeftBottomPoint").unwrap().0);
        bottom = map
            .get("RightBottomPoint")
            .unwrap()
            .1
            .min(map.get("LeftBottomPoint").unwrap().1);
        right = map
            .get("RightTopPoint")
            .unwrap()
            .0
            .max(map.get("RightBottomPoint").unwrap().0);
        top = map
            .get("LeftTopPoint")
            .unwrap()
            .1
            .max(map.get("RightTopPoint").unwrap().1);
    } else {
        let mut keys: Vec<String> = Vec::with_capacity(8);
        let mut values: Vec<f64> = Vec::with_capacity(8);
        let elements = vec![
            "TopLeftLatitude",
            "TopLeftLongitude",
            "TopRightLatitude",
            "TopRightLongitude",
            "BottomRightLatitude",
            "BottomRightLongitude",
            "BottomLeftLatitude",
            "BottomLeftLongitude",
        ];
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    if (&elements).contains(&name.local_name.as_str()) {
                        keys.push(name.local_name);
                    }
                }
                Ok(XmlEvent::Characters(chars)) => {
                    if values.len() != keys.len() {
                        values.push(chars.parse::<f64>().unwrap());
                    }
                    if values.len() == 8 {
                        break;
                    }
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
        let map: HashMap<String, f64> = keys.into_iter().zip(values.into_iter()).collect();
        left = map
            .get("TopLeftLongitude")
            .unwrap()
            .min(*map.get("BottomLeftLongitude").unwrap());
        bottom = map
            .get("BottomRightLatitude")
            .unwrap()
            .min(*map.get("BottomLeftLatitude").unwrap());
        right = map
            .get("TopRightLongitude")
            .unwrap()
            .max(*map.get("BottomRightLongitude").unwrap());
        top = map
            .get("TopLeftLatitude")
            .unwrap()
            .max(*map.get("TopRightLatitude").unwrap());
    }
    [left, bottom, right, top]
}

/// Parse input args.
pub struct ParamsFindIntersected<'a> {
    in_raster: &'a str,
    out_file: &'a str,
    image_dir: &'a str,
    formats: Vec<&'a str>,
}

use clap;
impl<'a> ParamsFindIntersected<'a> {
    pub fn new(matches: &'a clap::ArgMatches) -> Result<ParamsFindIntersected<'a>, &'static str> {
        let mut in_raster: &'a str = "";
        if let Some(param) = matches.value_of("in_raster") {
            in_raster = param;
        }

        let mut out_file: &'a str = "";
        if let Some(param) = matches.value_of("out_file") {
            out_file = param;
        }

        let mut image_dir: &'a str = "";
        if let Some(param) = matches.value_of("image_dir") {
            image_dir = param;
        }

        let mut formats: Vec<&'a str> = Vec::new();
        if let Some(param) = matches.value_of("formats") {
            formats = param.trim().split_whitespace().collect();
        }

        Ok(ParamsFindIntersected {
            in_raster: in_raster,
            out_file: out_file,
            image_dir: image_dir,
            formats: formats,
        })
    }
}

/// main program.
pub fn run_find_intersected(args: ParamsFindIntersected) -> Result<(), Box<dyn std::error::Error>> {
    let out_file = args.out_file;
    let img = args.in_raster;
    let src_bound = ImageBoundary::construct_from_raster(&img).expect("Get image boundary error.");

    let ref_dir = args.image_dir;

    // All files with given extension.
    let mut ref_paths: Vec<Paths> = Vec::new();
    for ext in &args.formats {
        ref_paths.push(get_files(&ref_dir, ext).unwrap())
    }

    // construt boundaries acording to all the images.
    let mut bouds: Vec<ImageBoundary> = Vec::new();
    for paths in ref_paths {
        for entry in paths {
            match entry {
                Ok(p) => {
                    bouds.push(ImageBoundary::construct_from_raster(p.to_str().unwrap()).unwrap());
                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    // Build rtree.
    let tree = RTree::bulk_load(bouds);

    // find intersected
    let intersected = tree.locate_in_envelope_intersecting(&src_bound.envelope());
    // file names
    let imgs = intersected.map(|c| &c.path);

    // write to a file.
    check_parent_dir(&out_file).unwrap();
    match File::create(out_file) {
        Ok(f) => {
            let mut writer = BufWriter::new(f);
            for t in imgs {
                println!("{}", t);
                writer.write((t.to_owned() + "\n").as_bytes())?;
            }
        }
        Err(e) => panic!("Problem creating the file: {:?}", e),
    }

    Ok(())
}
