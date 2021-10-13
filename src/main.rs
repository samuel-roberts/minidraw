use image::Rgba;
use obj::Obj;
use nalgebra::{Point2};

mod renderer;

use renderer::Renderer;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 1000;

fn main() {
    let mut renderer = Renderer::new(WIDTH, HEIGHT);

    let data = Obj::load("test.obj").unwrap().data;

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
                    .map(|p| Point2::<i32>::new(
                        ((p[0] + 1.0) * ((WIDTH as f32) / 2.0)) as i32,
                        ((p[1] + 1.0) * ((HEIGHT as f32) / 2.0)) as i32
                    ))
                    .collect();

                //renderer.line(positions[0], positions[1], Rgba([0, 0, 0, 255]));
                //renderer.line(positions[1], positions[2], Rgba([0, 0, 0, 255]));
                //renderer.line(positions[2], positions[0], Rgba([0, 0, 0, 255]));
                renderer.triangle(positions[0], positions[1], positions[2], Rgba([0, 0, 0, 255]));
            }
        }
    }

    renderer.save("output.png").unwrap();
}
