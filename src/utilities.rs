use image::Rgba;
use nalgebra::{Point3, Vector2, Vector3};
use rand::Rng;
use std::cmp;

///
#[inline]
pub fn clamp<T: cmp::Ord>(t: T, min: T, max: T) -> T {
    cmp::min(max, cmp::max(min, t))
}

///
#[inline]
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
#[inline]
pub fn sigmoid(value: f32) -> f32 {
    1.0 / (1.0 + (-value).exp())
}

///
pub fn barycentric(
    p0: Point3<f32>,
    p1: Point3<f32>,
    p2: Point3<f32>,
    p: Point3<f32>,
) -> Vector3<f32> {
    let v0 = p1 - p0;
    let v1 = p2 - p0;
    let v2 = p - p0;

    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    let denominator = (d00 * d11) - (d01 * d01);

    let v = ((d11 * d20) - (d01 * d21)) / denominator;
    let w = ((d00 * d21) - (d01 * d20)) / denominator;
    let u = 1.0 - v - w;

    Vector3::<f32>::new(u, v, w)
}

///
pub fn random_colour() -> Rgba<u8> {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..=255);
    let g = rng.gen_range(0..=255);
    let b = rng.gen_range(0..=255);
    Rgba([r, g, b, 255])
}
