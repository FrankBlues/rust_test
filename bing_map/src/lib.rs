use std::path::Path;
use std::time::SystemTime;

mod bing_tiles_system;
pub use crate::bing_tiles_system::tiles_system::TileSystem;

mod download_utils;
pub use download_utils::download_utils as download_util;

mod io_utils;
pub use io_utils::write_string_to_text;

mod image_process;
pub use image_process::merge_tiles;

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
        assert_eq!(
            (TileSystem::ground_resolution(0., 14) * 10000.).round() / 10000.,
            9.5546
        )
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

/// Parse the input parameters.
pub struct Config {
    lon0: f64,
    lat0: f64,
    lon1: f64,
    lat1: f64,
    zoom: u8,
    tiles_dir: String,
    out_png: String,
}

impl Config {
    pub fn new (matches: clap::ArgMatches) -> Result<Config, &'static str> {
        let (mut lon0, mut lat0) = (0., 0.);
        if let Some(ul_lonlat) = matches.value_of("ul_lonlat") {
            let mut lonlat0 = ul_lonlat.trim().split_whitespace();
            lon0 = lonlat0.next()
                    .expect("Failed parsing ul_lonlat")
                    .parse().expect("Failed parsing lon0 from ul_lonlat.");
            lat0 = lonlat0.next()
                        .expect("Failed parsing ul_lonlat")
                        .parse().expect("Failed parsing lat0 from ul_lonlat.");
        }
        let (mut lon1, mut lat1) = (0., 0.);
        if let Some(br_lonlat) = matches.value_of("br_lonlat") {
            let mut lonlat1 = br_lonlat.trim().split_whitespace();
            lon1 = lonlat1.next()
                    .expect("Failed parsing br_lonlat")
                    .parse().expect("Failed parsing lon1 from br_lonlat.");
            lat1 = lonlat1.next()
                    .expect("Failed parsing br_lonlat")
                    .parse().expect("Failed parsing lat1 from br_lonlat.");
        }
        let mut zoom: u8 = 18;
        if let Some(z) = matches.value_of("zoom_level") {
            println!("{}", z);
            zoom = z.trim().parse().unwrap();
        }
        let mut tiles_dir = String::new();
        if let Some(tile_dir) = matches.value_of("tiles_dir") {
            tiles_dir = String::from(tile_dir);
        }
        let mut out_png = String::new();
        if let Some(output) = matches.value_of("output") {
            out_png = String::from(output);
        }

        Ok(Config {
            lon0, lat0, lon1, lat1, zoom, tiles_dir, out_png
        })
    }
}

/// Main program to run.
pub async fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {

    let level = config.zoom;
    let tile_dir = Path::new(&config.tiles_dir).join((&level).to_string());
    let out_png = config.out_png;
    let world_file = out_png.replace(".png", ".pgw");

    let te = TilesExtent::new(config.lon0, config.lat0, config.lon1, config.lat1, level);
    let urls_files = te.construct_download_params(&tile_dir);
    let (tile0, tile1) = te.tile_extent();
    let world_file_content = te.gen_world_file_content(&tile0);

    //download concurrently
    println!("Download start!");
    let st_time = SystemTime::now();
    download_util::download_files_async(&urls_files).await;
    let lt_time = SystemTime::now();
    println!(
        "{} tiles downloaded, spend {:?}",
        &urls_files.len(),
        SystemTime::duration_since(&lt_time, st_time).unwrap()
    );

    println!("Merging the tiles.");
    merge_tiles(tile0, tile1, out_png, &tile_dir)?;

    println!("Generate world file.");
    write_string_to_text(&world_file, world_file_content)?;

    Ok(())
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

    pub fn tile_extent(&self) -> ((usize, usize), (usize, usize)) {
        // upper left tile number
        let ul_tile = TilesExtent::tilexy_from_lonlat(self.lon0, self.lat0, self.zoom);
        // bottom right tile number
        let br_tile = TilesExtent::tilexy_from_lonlat(self.lon1, self.lat1, self.zoom);
        (ul_tile, br_tile)
    }

    pub fn cal_tile_lonlat(tile_x: usize, tile_y: usize, zoom: u8) -> (f64, f64) {
        let (x, y) = TileSystem::tile_xy2xy(tile_x, tile_y);
        TileSystem::xy2latlon(x, y, zoom)
    }

    pub fn tiles(&self) -> Vec<(usize, usize)> {
        let mut _tiles: Vec<(usize, usize)> = Vec::new();
        let (ul_tile, br_tile) = self.tile_extent();
        for i in ul_tile.0..=br_tile.0 {
            for j in ul_tile.1..=br_tile.1 {
                _tiles.push((i, j));
            }
        }
        _tiles
    }

    fn tiles2quad_keys(&self, tiles: &Vec<(usize, usize)>) -> Vec<String> {
        tiles
            .iter()
            .map(|tup| TileSystem::tile_xy2quad_key(tup.0, tup.1, self.zoom))
            .collect()
    }

    /// 计算quad_keys
    pub fn quad_keys(&self) -> Vec<String> {
        let _tiles = self.tiles();
        self.tiles2quad_keys(&_tiles)
    }

    /// 构建下载参数<url地址, 本地路径>
    ///
    pub fn construct_download_params(
        &self,
        tile_dir: &std::path::PathBuf,
    ) -> Vec<(String, std::path::PathBuf)> {
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

    /// Generate world file content as String.
    pub fn gen_world_file_content(&self, tile: &(usize, usize)) -> String {
        let (lat, lon) = TilesExtent::cal_tile_lonlat(tile.0, tile.1, self.zoom);
        let (mercator_x, mercator_y) = TileSystem::latlon2mercator(lat, lon);
        let res = TileSystem::ground_resolution(0., self.zoom);
        format!("{:.7}\n0\n0\n{:.7}\n{:.7}\n{:.7}", res, -res, mercator_x, mercator_y)
    }
}
