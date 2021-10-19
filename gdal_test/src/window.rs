
pub struct Window {
    pub position: (isize, isize),
    pub size: (usize, usize),
}

pub fn calculate_window(left: f64, bottom: f64, right: f64, top: f64, geo_transform: &[f64; 6]) -> Window {

    let x = ((left - geo_transform[0]) / geo_transform[1]).round() as isize;
    let y = ((top - geo_transform[3]) / geo_transform[5]).round() as isize;
    let width = ((right - left) / geo_transform[1]).round() as usize;
    let height = ((bottom - top) / geo_transform[5]).round() as usize;
    Window {
        position: (x, y),
        size: (width, height),
    }
}

