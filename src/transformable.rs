use nalgebra::Vector3;

pub trait Transformable {
    ///
    fn translate(&mut self, delta: Vector3<f32>);

    ///
    fn rotate(&mut self, x: f32, y: f32, z: f32);

    ///
    fn scale(&mut self, factor: f32);
}