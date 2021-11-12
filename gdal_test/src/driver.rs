/// guess driver by file extension
pub fn guess_driver_by_name(file_name: &str) -> Option<&str> {

    if let Some(ext) = get_extension(file_name) {
        let ext = ext.to_lowercase();
        let ext1 = ext.as_str();
        // Vector
        if ["json", "geojson"].contains(&ext1){
            return Some("GeoJSON");
        } else if ["shp", "shx", "dbf"].contains(&ext1) {
            return Some("ESRI Shapefile");
        }

        // Raster
        if ["tif", "tiff"].contains(&ext1) {
            return Some("GTiff");
        } else if [".img", ".ige"].contains(&ext1) {
            return Some("HFA");
        }
    }
    None
}

pub fn get_extension(file_name: &str) -> Option<&str>{
    if file_name.contains(".") {
        let v: Vec<&str> = file_name.split(".").collect();
        match v.last() {
            Some(last) => return Some(*last),
            None => return None,
        }
    }
    None
}