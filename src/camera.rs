use nalgebra::{Matrix4, Point3, Vector3};

use crate::transformable::Transformable;

pub struct Camera {
    position: Point3<f32>,
    projection_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    view_projection_matrix: Matrix4<f32>,
}

impl Camera {
    ///
    pub fn new(aspect_ratio: f32, field_of_view: f32, z_near: f32, z_far: f32) -> Camera {
        let projection =
            Matrix4::<f32>::new_perspective(aspect_ratio, field_of_view, z_near, z_far);

        let mut camera = Camera {
            position: Point3::<f32>::origin(),
            projection_matrix: projection,
            view_matrix: Matrix4::<f32>::identity(),
            view_projection_matrix: Matrix4::<f32>::identity(),
        };

        camera.update_camera();
        camera
    }

    ///
    #[inline]
    pub fn get_position(&self) -> &Point3<f32> {
        &self.position
    }

    ///
    pub fn set_position(&mut self, position: &Point3<f32>) {
        self.position = *position;
        self.update_camera();
    }

    ///
    pub fn get_direction(&self) -> Vector3<f32> {
        -self
            .view_matrix
            .try_inverse()
            .unwrap()
            .transform_vector(&Vector3::<f32>::z_axis())
    }

    ///
    #[inline]
    pub fn get_view_projection_matrix(&self) -> &Matrix4<f32> {
        &self.view_projection_matrix
    }

    ///
    pub fn look_at(&mut self, target: &Point3<f32>) {
        self.view_matrix =
            Matrix4::<f32>::look_at_lh(&self.position, target, &Vector3::<f32>::z_axis());
        self.update_camera();
    }

    ///
    fn update_camera(&mut self) {
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;
    }
}

impl Transformable for Camera {
    ///
    fn translate(&mut self, delta: Vector3<f32>) {
        self.view_matrix *= Matrix4::<f32>::new_translation(&delta); // TODO Test
    }

    ///
    fn rotate(&mut self, x: f32, y: f32, z: f32) {
        let rot_x = Vector3::x() * x;
        let rot_y = Vector3::y() * y;
        let rot_z = Vector3::z() * z;
        self.view_matrix *= Matrix4::<f32>::new_rotation(rot_x + rot_y + rot_z);
        // TODO Test
    }

    ///
    fn scale(&mut self, _: f32) {
        self.update_camera();
    }
}
