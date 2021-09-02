pub mod bing_tiles_system;
use crate::bing_tiles_system::tiles_system::TileSystem;

mod download_utils;
pub use download_utils::download_utils as download_util;

mod io_utils;
pub use io_utils::get_files as get_files;

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

    fn tiles2quad_keys(&self, tiles: &Vec<(usize, usize)>) -> Vec<String>{
        tiles.iter().map(|tup| TileSystem::tile_xy2quad_key(tup.0, tup.1, self.zoom)).collect()
    }

    pub fn quad_keys(&self) -> Vec<String> {
        let _tiles = self.tiles();
        self.tiles2quad_keys(&_tiles)
    }

    pub fn construct_download_params(&self, tile_dir: &std::path::PathBuf) -> Vec<(String, std::path::PathBuf)>{
        let tiles = self.tiles();
        let quad_keys = self.tiles2quad_keys(&tiles);

        let mut urls_files: Vec<(String, std::path::PathBuf)> = Vec::with_capacity(quad_keys.len());
        // z/x/y.jpeg
        for (i, (x, y)) in tiles.iter().enumerate() {
            let q = &quad_keys[i];
            let url = download_util::constuct_url(q, "a");
            let path = tile_dir.join(x.to_string()).join(y.to_string() + ".jpeg");
            if !path.parent().unwrap().exists() {
                std::fs::create_dir_all(&path.parent().unwrap()).unwrap();
            }
            urls_files.push((url, path));
        }
        urls_files
    }

}