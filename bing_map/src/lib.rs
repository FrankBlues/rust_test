use std::f64::consts::PI;

const EARTH_RADIUS: f64 = 6378137.;  
const MIN_LATITUDE: f64 = -85.05112878;  
const MAX_LATITUDE: f64 = 85.05112878;  
const MIN_LONGITUDE: f64 = -180.;  
const MAX_LONGITUDE: f64 = 180.;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_test() {
        assert_eq!(TileSystem::map_size(2), 1024)
    }

    #[test]
    fn min_test() {
        assert_eq!(min(2.1, 3.), 2.1)
    }

    #[test]
    fn max_test() {
        assert_eq!(max(2.1, 3.), 3.)
    }

    #[test]
    fn res_test() {
        assert_eq!((TileSystem::ground_resolution(0., 14)*10000.).round()/10000., 9.5546)
    }

    #[test]
    fn latlon2xy_test() {
        assert_eq!(TileSystem::latlon2xy(0., 0., 1), (256, 256));
        assert_eq!(TileSystem::latlon2xy(90., 0., 1), (256, 0));
        assert_eq!(TileSystem::latlon2xy(-90., 200., 1), (511, 511));
        assert_eq!(TileSystem::latlon2xy(-90., 200., 3), (2047, 2047));
    }

    #[test]
    fn xy2latlon_test() {
        assert_eq!(TileSystem::xy2latlon(256, 256, 1), (0., 0.));
        assert_eq!(TileSystem::xy2latlon(0, 0, 1), (85.05112877980659, -180.));
        // assert_eq!(TileSystem::xy2latlon(511, 511, 1), (-85.05112877980659, 180.));
        // assert_eq!(TileSystem::xy2latlon(2047, 2047, 1), (-85.05112877980659, 180.));
    }

    #[test]
    fn tile_xy2quad_key_test() {
        assert_eq!(TileSystem::tile_xy2quad_key(0, 0, 1), String::from("0"));
        assert_eq!(TileSystem::tile_xy2quad_key(1, 1, 1), String::from("3"));
        assert_eq!(TileSystem::tile_xy2quad_key(1, 2, 2), String::from("21"));
        assert_eq!(TileSystem::tile_xy2quad_key(0, 0, 3), String::from("000"));
        assert_eq!(TileSystem::tile_xy2quad_key(7, 7, 3), String::from("333"));
    }

    #[test]
    fn quad_key2tile_xy_test() {
        assert_eq!(TileSystem::quad_key2tile_xy(String::from("330")), (6, 6, 3));
        assert_eq!(TileSystem::quad_key2tile_xy(String::from("21")), (1, 2, 2));
        assert_eq!(TileSystem::quad_key2tile_xy(String::from("3")), (1, 1, 1));
    }
}


fn min(x: f64, y: f64) -> f64{
    if x < y {x} else {y}
}

fn max(x: f64, y: f64) -> f64{
    if x > y {x} else {y}
}

pub struct TileSystem;
impl TileSystem {
 
    /// Clips a number to the specified minimum and maximum values. 
    fn clip(n: f64, min_value: f64, max_value: f64) -> f64{
        min(max(n, min_value), max_value)
    }

    /// Determines the map width and height (in pixels) at a specified level  
    /// of detail.
    pub fn map_size(levels: u8) -> usize {
        256 << levels
    }

    /// Determines the ground resolution (in meters per pixel) at a specified  
    /// latitude and level of detail. 
    pub fn ground_resolution(latitude: f64, levels: u8) -> f64 {
        let latitude = TileSystem::clip(latitude, MIN_LATITUDE, MAX_LATITUDE);
        (latitude * PI / 180.).cos() * 2. * PI * EARTH_RADIUS / (TileSystem::map_size(levels) as f64)
    }

    /// Determines the map scale at a specified latitude, level of detail,  
    /// and screen resolution.
    pub fn map_scale(latitude: f64, levels: u8, screen_dpi: u16) -> f64 {
        TileSystem::ground_resolution(latitude, levels) * (screen_dpi as f64) / 0.0254
    }

    /// Converts a point from latitude/longitude WGS-84 coordinates (in degrees)  
    /// into pixel XY coordinates at a specified level of detail.
    pub fn latlon2xy(latitude: f64, longitude: f64, levels: u8) -> (usize, usize) {
        let lat = TileSystem::clip(latitude, MIN_LATITUDE, MAX_LATITUDE);
        let long = TileSystem::clip(longitude, MIN_LONGITUDE, MAX_LONGITUDE);
        let x = (long + 180.) / 360.;
        let sin_latitude = (lat * PI / 180.).sin();
        let y = 0.5 - ((1. + sin_latitude) / (1. - sin_latitude)).ln()/(4. * PI);
        let map_size = TileSystem::map_size(levels) as f64;
        let pixel_x = TileSystem::clip(x * &map_size + 0.5, 0., &map_size - 1.);
        let pixel_y = TileSystem::clip(y * &map_size + 0.5, 0., &map_size - 1.);

        (pixel_x as usize, pixel_y as usize)
    }

    /// Converts a pixel from pixel XY coordinates at a specified level of detail  
    /// into latitude/longitude WGS-84 coordinates (in degrees).
    pub fn xy2latlon(pixel_x: usize, pixel_y: usize, levels: u8) -> (f64, f64){
        let map_size = TileSystem::map_size(levels) as f64;
        let x = TileSystem::clip(pixel_x as f64, 0., &map_size - 1.)/&map_size - 0.5;
        let y = 0.5 - TileSystem::clip(pixel_y as f64, 0., &map_size - 1.)/&map_size;
        let latitude = 90. - 360. * ((-y * 2. * PI).exp().atan()) / PI;
        (latitude, 360. * x)
    }

    /// Converts pixel XY coordinates into tile XY coordinates of the tile containing  
    /// the specified pixel.
    pub fn xy2tile_xy(pixel_x: usize, pixel_y: usize) -> (usize, usize) {
        (pixel_x / 256, pixel_y / 256)
    }

    /// Converts tile XY coordinates into pixel XY coordinates of the upper-left pixel  
    /// of the specified tile.
    pub fn tile_xy2xy(tile_x: usize, tile_y: usize) -> (usize, usize) {
        (tile_x * 256, tile_y * 256)
    }

    /// Converts tile XY coordinates into a QuadKey at a specified level of detail.
    pub fn tile_xy2quad_key(tile_x: usize, tile_y: usize, levels: u8) -> String {
        let mut quad_key = String::new();
        for i in (1..=levels).rev() {
            let mut digit = '0' as u8;
            let mask = 1 << (i - 1);
            if tile_x & (&mask) != 0 {
                digit += 1;
            }

            if tile_y & (&mask) != 0 {
                digit += 2;
                // digit += 1;
            }
            quad_key.push(digit as char)
        }

        quad_key
    }

    /// Converts a QuadKey into tile XY coordinates.
    pub fn quad_key2tile_xy(quad_key: String) -> (usize, usize, u8) {
        let  (mut tile_x, mut tile_y) = (0, 0);
        let levels = quad_key.len();
        for i in (1..=levels).rev() {
            let mask = 1 << (i - 1);
            match &quad_key.chars().nth(levels - i) {
                Some('0') => continue,
                Some('1') => {
                    tile_x |= &mask;
                    continue;
                }
                Some('2') => {
                    tile_y |= &mask;
                    continue;
                }
                Some('3') => {
                    tile_x |= &mask;
                    tile_y |= &mask;
                    println!("{}, {}", tile_x, tile_y);
                    continue;
                }
                _ => panic!("Invalid QuadKey digit sequence."),
            }
        }
        (tile_x, tile_y, levels as u8)
    }
}

pub struct TilesExtent {
    lon0: f64,
    lat0: f64,
    lon1: f64,
    lat1: f64,
    zoom: u8,
}

impl TilesExtent {
    pub fn new(lon0: f64, lat0: f64, lon1: f64, lat1: f64, zoom: u8) -> TilesExtent {
        assert!(lon0 < lon1 && lat0 > lat1);
        TilesExtent {
            lon0: lon0,
            lat0: lat0,
            lon1: lon1,
            lat1: lat1,
            zoom: zoom,
        }
    }

    fn tilexy_from_lonlat(lon: f64, lat: f64, zoom: u8) -> (usize, usize) {
        let (_x, _y) = TileSystem::latlon2xy(lat, lon, zoom);
        TileSystem::xy2tile_xy(_x, _y)
    }

    pub fn tiles(&self) -> Vec<(usize, usize)> {
        let mut _tiles: Vec<(usize, usize)> = Vec::new();
        let ul_tile = TilesExtent::tilexy_from_lonlat(self.lon0, self.lat0, self.zoom);
        let br_tile = TilesExtent::tilexy_from_lonlat(self.lon1, self.lat1, self.zoom);
        for i in ul_tile.0..=br_tile.0 {
            for j in ul_tile.1..=br_tile.1 {
                _tiles.push((i, j));
            }
        }
        _tiles
    }

    pub fn quad_keys(&self) -> Vec<String> {
        let _tiles = self.tiles();
        _tiles.iter().map(|tup| TileSystem::tile_xy2quad_key(tup.0, tup.1, self.zoom)).collect()
    }

}