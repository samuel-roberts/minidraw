use image::Rgba;
use nalgebra::Matrix4;

use super::vertex::Vertex;

pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
    pub colour: Rgba<u8>,
}

impl Triangle {
    ///
    pub fn transform(&self, transform: &Matrix4<f32>) -> Triangle {
        Triangle {
            a: self.a.transform(&transform),
            b: self.b.transform(&transform),
            c: self.c.transform(&transform),
            colour: self.colour,
        }
    }
}
