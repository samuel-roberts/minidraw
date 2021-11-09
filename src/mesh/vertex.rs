use image::Rgba;
use nalgebra::{Matrix4, Point3, Vector2, Vector3};

pub struct Vertex {
    pub position: Point3<f32>,
    pub normal: Option<Vector3<f32>>,
    pub uv: Option<Vector2<f32>>,
    pub colour: Option<Rgba<u8>>,
}

impl Vertex {
    ///
    pub fn transform(&self, transform: &Matrix4<f32>) -> Vertex {
        Vertex {
            position: transform.transform_point(&self.position),
            normal: match self.normal {
                Some(normal) => Some(transform.transform_vector(&normal)),
                None => None,
            },
            uv: self.uv,
            colour: self.colour,
        }
    }
}
