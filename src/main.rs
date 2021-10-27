
use drawable::Drawable;
use nalgebra::{Matrix4, Point3, Vector3};

mod renderer;
mod utilities;
mod mesh;
mod drawable;

use renderer::Renderer;
use mesh::Mesh;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let view = Matrix4::<f32>::look_at_lh(
        &Point3::<f32>::new(25.0, 25.0, 25.0),
        &Point3::<f32>::origin(),
        &Vector3::<f32>::z_axis(),
    );
    let projection = Matrix4::<f32>::new_perspective(
        (renderer.get_width() as f32) / (renderer.get_height() as f32),
        std::f32::consts::PI / 2.0,
        1e-5,
        1e5,
    );

    renderer.push_view_matrix(view);
    renderer.push_projection_matrix(projection);

    let mesh = Mesh::load_obj("models/calibration.obj").expect("Failed to load model");
    mesh.draw(&mut renderer);

    renderer.save("output.png").unwrap();
}
