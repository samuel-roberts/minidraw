use image::Rgba;
use nalgebra::{Point2, Vector3};
use rand::Rng;

///
pub fn barycentric(
    p0: Point2<i32>,
    p1: Point2<i32>,
    p2: Point2<i32>,
    p: Point2<i32>,
) -> Option<Vector3<f32>> {
    let a = Vector3::<f32>::new(
        (p2.x - p0.x) as f32,
        (p1.x - p0.x) as f32,
        (p0.x - p.x) as f32,
    );
    let b = Vector3::<f32>::new(
        (p2.y - p0.y) as f32,
        (p1.y - p0.y) as f32,
        (p0.y - p.y) as f32,
    );
    let u = a.cross(&b);

    if u.z.abs() < 1.0 {
        None
    } else {
        let v = Vector3::<f32>::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z);
        Some(v)
    }
}

pub fn random_colour() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    Rgba([r, g, b, 255])
}
