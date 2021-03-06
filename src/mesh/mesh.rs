use image::Rgba;
use nalgebra::{Matrix4, Point3, Vector3};
use obj::Obj;

use crate::drawable::Drawable;
use crate::mesh::triangle::Triangle;
use crate::mesh::vertex::Vertex;
use crate::renderer::Renderer;
use crate::transformable::Transformable;

pub struct Mesh {
    geometry: Vec<Triangle>,
    transform: Matrix4<f32>,
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
                        a: Vertex {
                            position: positions[0],
                            normal: None,
                            uv: None,
                            colour: None,
                        },
                        b: Vertex {
                            position: positions[1],
                            normal: None,
                            uv: None,
                            colour: None,
                        },
                        c: Vertex {
                            position: positions[2],
                            normal: None,
                            uv: None,
                            colour: None,
                        },
                        colour: Rgba([255, 255, 255, 255]),
                    });
                }
            }
        }

        return Ok(Mesh {
            geometry: triangles,
            transform: Matrix4::<f32>::identity(),
        });
    }
}

impl Drawable for Mesh {
    ///
    fn draw(&self, renderer: &mut Renderer) {
        for triangle in &self.geometry {
            let transformed_triangle = triangle.transform(&self.transform);
            renderer.triangle(&transformed_triangle);
        }
    }

    ///
    fn draw_wireframe(&self, renderer: &mut Renderer) {
        for triangle in &self.geometry {
            let transformed_triangle = triangle.transform(&self.transform);
            renderer.line(
                transformed_triangle.a.position,
                transformed_triangle.b.position,
                transformed_triangle.colour,
            );
            renderer.line(
                transformed_triangle.b.position,
                transformed_triangle.c.position,
                transformed_triangle.colour,
            );
            renderer.line(
                transformed_triangle.c.position,
                transformed_triangle.a.position,
                transformed_triangle.colour,
            );
        }
    }
}

impl Transformable for Mesh {
    ///
    fn translate(&mut self, delta: Vector3<f32>) {
        self.transform *= Matrix4::<f32>::new_translation(&delta);
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
        self.transform *= Matrix4::<f32>::new_scaling(factor);
    }
}
