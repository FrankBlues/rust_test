pub mod tiles_system {
    use std::f64::consts::PI;

    const EARTH_RADIUS: f64 = 6378137.;
    const MIN_LATITUDE: f64 = -85.05112878;
    const MAX_LATITUDE: f64 = 85.05112878;
    const MIN_LONGITUDE: f64 = -180.;
    const MAX_LONGITUDE: f64 = 180.;
    const WEB_MERCATOR_R: f64 = PI * EARTH_RADIUS;
    fn min(x: f64, y: f64) -> f64 {
        if x < y {
            x
        } else {
            y
        }
    }

    fn max(x: f64, y: f64) -> f64 {
        if x > y {
            x
        } else {
            y
        }
    }
    pub struct TileSystem;
    impl TileSystem {
        /// Clips a number to the specified minimum and maximum values.
        fn clip(n: f64, min_value: f64, max_value: f64) -> f64 {
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
            (latitude * PI / 180.).cos() * 2. * PI * EARTH_RADIUS
                / (TileSystem::map_size(levels) as f64)
        }

        /// Converts a point from latitude/longitude WGS-84 coordinates (in degrees)  
        /// into web mercator coordinates.
        pub fn latlon2mercator(latitude: f64, longitude: f64) -> (f64, f64) {
            let lat = TileSystem::clip(latitude, MIN_LATITUDE, MAX_LATITUDE);
            let long = TileSystem::clip(longitude, MIN_LONGITUDE, MAX_LONGITUDE);
            let x = long * WEB_MERCATOR_R / 180.;
            let y = ((90.0 + lat) * PI / 360.).tan().ln() / (PI / 180.);
            let y = y * WEB_MERCATOR_R / 180.;
            (x, y)
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
            let y = 0.5 - ((1. + sin_latitude) / (1. - sin_latitude)).ln() / (4. * PI);
            let map_size = TileSystem::map_size(levels) as f64;
            let pixel_x = TileSystem::clip(x * &map_size + 0.5, 0., &map_size - 1.);
            let pixel_y = TileSystem::clip(y * &map_size + 0.5, 0., &map_size - 1.);

            (pixel_x as usize, pixel_y as usize)
        }

        /// Converts a pixel from pixel XY coordinates at a specified level of detail  
        /// into latitude/longitude WGS-84 coordinates (in degrees).
        pub fn xy2latlon(pixel_x: usize, pixel_y: usize, levels: u8) -> (f64, f64) {
            let map_size = TileSystem::map_size(levels) as f64;
            let x = TileSystem::clip(pixel_x as f64, 0., &map_size - 1.) / &map_size - 0.5;
            let y = 0.5 - TileSystem::clip(pixel_y as f64, 0., &map_size - 1.) / &map_size;
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
            let (mut tile_x, mut tile_y) = (0, 0);
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
}
