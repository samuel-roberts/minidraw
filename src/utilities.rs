use image::Rgba;
use nalgebra::{Point3, Vector3};
use rand::Rng;
use std::cmp;

///
pub fn clamp<T: cmp::Ord>(t: T, min: T, max: T) -> T {
    cmp::min(max, cmp::max(min, t))
}

///
pub fn clamp_f32(t: f32, min: f32, max: f32) -> f32 {
    if t < min {
        min
    } else if t > max {
        max
    } else {
        t
    }
}

///
pub fn barycentric(
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    p: Point3<f32>,
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

///
pub fn random_colour() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    Rgba([r, g, b, 255])
}
