use image::Rgba;
use nalgebra::{Point3, Vector2, Vector3};

pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Option<Vector3<f32>>,
    pub uv: Option<Vector2<f32>>,
    pub colour: Option<Rgba<u8>>,
}
