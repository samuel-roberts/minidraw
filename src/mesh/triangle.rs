use image::Rgba;

use super::vertex::Vertex;

pub struct Triangle {
    pub a: Vertex,
    pub b: Vertex,
    pub c: Vertex,
    pub colour: Rgba<u8>,
}
