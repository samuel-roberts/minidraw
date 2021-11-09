use nalgebra::{Matrix4, Point3, Vector3};

pub struct Camera {
    direction: Vector3<f32>,
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
            direction: *Vector3::<f32>::z_axis(),
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
    #[inline]
    pub fn get_direction(&self) -> &Vector3<f32> {
        &self.direction
    }

    ///
    #[inline]
    pub fn get_view_projection_matrix(&self) -> &Matrix4<f32> {
        &self.view_projection_matrix
    }

    ///
    pub fn look_at(&mut self, eye: &Point3<f32>, target: &Point3<f32>, up: &Vector3<f32>) {
        self.view_matrix = Matrix4::<f32>::look_at_lh(eye, target, up);
        self.update_camera();
    }

    ///
    fn update_camera(&mut self) {
        self.view_projection_matrix = self.projection_matrix * self.view_matrix;

        self.direction = -self
            .view_matrix
            .try_inverse()
            .unwrap()
            .transform_vector(&Vector3::<f32>::z_axis());

        self.position = self
            .view_matrix
            .try_inverse()
            .unwrap()
            .transform_point(&Point3::<f32>::origin());
    }
}
