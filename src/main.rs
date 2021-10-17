use image::Rgba;
use nalgebra::{Matrix4, Point3, Vector3};
use obj::Obj;

mod renderer;
mod utilities;

use renderer::Renderer;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let model = Matrix4::<f32>::identity();
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

    renderer.push_model_matrix(model);
    renderer.push_view_matrix(view);
    renderer.push_projection_matrix(projection);

    let data = Obj::load("models/calibration.obj").unwrap().data;

    for object in &data.objects {
        for group in &object.groups {
            for polygon in &group.polys {
                // Ignore polygons that are not triangles
                if polygon.0.len() != 3 {
                    continue;
                }

                // Get the indicies of the points of the triangle
                let idx0 = polygon.0[0].0;
                let idx1 = polygon.0[1].0;
                let idx2 = polygon.0[2].0;

                let indices = vec![idx0, idx1, idx2];
                let positions: Vec<_> = indices
                    .iter()
                    .map(|i| data.position[*i])
                    .map(|p| Point3::<f32>::new(p[0], p[1], p[2]))
                    .collect();

                //renderer.line(positions[0], positions[1], Rgba([0, 0, 0, 255]));
                //renderer.line(positions[1], positions[2], Rgba([0, 0, 0, 255]));
                //renderer.line(positions[2], positions[0], Rgba([0, 0, 0, 255]));
                renderer.triangle(
                    positions[0],
                    positions[1],
                    positions[2],
                    Rgba([255, 255, 255, 255]),
                );
            }
        }
    }

    renderer.save("output.png").unwrap();
}
