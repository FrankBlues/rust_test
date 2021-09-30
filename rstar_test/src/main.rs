use rstar::{RTree, AABB, RTreeObject};
use rstar::primitives::Rectangle;

fn main() {
    let left_piece = AABB::from_corners([0.0, 0.0], [0.4, 1.0]);
    let right_piece = AABB::from_corners([0.6, 0.0], [1.0, 1.0]);
    let middle_piece = AABB::from_corners([0.25, 0.0], [0.75, 1.0]);
    let img1 = ImageBoundary::new(String::from("a"), 0.0, 0.0, 0.4, 1.0);
    let img2 = ImageBoundary::new(String::from("b"), 0.6, 0.0, 1.0, 1.0);
    let img3 = ImageBoundary::new(String::from("c"), 0.25, 0.0, 0.75, 1.0);

    let img = ImageBoundary::new(String::from("c"), 0.5, 0.0, 0.8, 1.0);

    let mut tree = RTree::bulk_load(vec![img1, img2, img3]);

    // let mut tree = RTree::<Rectangle<_>>::bulk_load(vec![
    // left_piece.into(),
    // right_piece.into(),
    // middle_piece.into(),
    // ]);

    let intersected = tree.locate_in_envelope_intersecting(&img.envelope());
    let imgs: Vec<&String> = intersected.map(|c| &c.path).collect();
    println!("{:?}", imgs);
    // for i in intersected {
    //     println!("{}", i.path);
    // }

    // The left piece should not intersect the right piece!
    // assert_eq!(elements_intersecting_left_piece.count(), 3);
}

struct ImageBoundary {
    path: String,
    left: f64,
    bottom: f64,
    right: f64,
    top: f64,
}

impl RTreeObject for ImageBoundary {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope
    {
        AABB::from_corners([self.left, self.bottom], [self.right, self.top])
    }
}

impl ImageBoundary {
    fn new(path: String, left: f64, bottom: f64, right: f64, top: f64,) -> ImageBoundary {
        ImageBoundary {
            path: path,
            left: left,
            bottom: bottom,
            right: right,
            top: top,
        }
    }
}
