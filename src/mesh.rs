
use image::Rgba;
use nalgebra::{Matrix4, Point3, Vector3};
use obj::Obj;

use crate::drawable::Drawable;
use crate::renderer::Renderer;
use crate::transformable::Transformable;

struct Triangle {
    a: Point3<f32>,
    b: Point3<f32>,
    c: Point3<f32>,
    colour: Rgba<u8>
}

pub struct Mesh {
    geometry: Vec<Triangle>,
    transform: Matrix4<f32>
}

impl Mesh {
    pub fn load_obj(filename: &str) -> Result<Mesh, String> {

        let data = Obj::load(filename).unwrap().data;
        let mut triangles = Vec::<Triangle>::new();

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
    
                    triangles.push(Triangle {
                        a: positions[0],
                        b: positions[1],
                        c: positions[2],
                        colour: Rgba([255, 255, 255, 255]),
                    });
                }
            }
        }

        return Ok(Mesh {
            geometry: triangles,
            transform: Matrix4::<f32>::identity()
        });
    }
}

impl Drawable for Mesh {
    ///
    fn draw(&self, renderer: &mut Renderer) {
        renderer.push_model_matrix(self.transform);
        for triangle in &self.geometry {
            renderer.triangle(triangle.a, triangle.b, triangle.c, triangle.colour);
        }
        renderer.pop_model_matrix();
    }
}

impl Transformable for Mesh {
    ///
    fn translate(&mut self, delta: Vector3<f32>) {
        todo!()
    }

    ///
    fn rotate(&mut self, x: f32, y: f32, z: f32) {
        let rot_x = Vector3::x() * x;
        let rot_y = Vector3::y() * y;
        let rot_z = Vector3::z() * z;
        self.transform *= Matrix4::<f32>::new_rotation(rot_x + rot_y + rot_z);
    }

    ///
    fn scale(&mut self, factor: f32) {
        todo!()
    }
}