use nalgebra::{Matrix4, Perspective3, Rotation3, Translation3, Unit, Vector3};
// use std::primitive::f32;

pub struct Camera {
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    local_matrix: Matrix4<f32>,
    target_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn new(fov: f32, near_clip: f32, far_clip: f32) -> Self {
        let mut camera = Camera {
            fov,
            near_clip,
            far_clip,
            local_matrix: Matrix4::<f32>::identity(),
            target_matrix: Matrix4::<f32>::identity(),
        };

        camera.zoom(-3.0);

        camera
    }
    pub fn from_focal_length(
        focal_length: f32,
        sensor_size: f32,
        near_clip: f32,
        far_clip: f32,
    ) -> Self {
        let mut camera = Camera::new(0.0, near_clip, far_clip);
        camera.set_focal_length(focal_length, sensor_size);
        camera
    }
    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.local_matrix * self.target_matrix
    }
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        Perspective3::new(1.0, self.fov, self.near_clip, self.far_clip).to_homogeneous()
    }
    pub fn zoom(&mut self, amount: f32) {
        let translation = Translation3::new(0.0, 0.0, amount);
        self.local_matrix *= translation.to_homogeneous();
    }
    pub fn rotate(&mut self, axis: Unit<Vector3<f32>>, angle: f32) {
        let rotation = Rotation3::from_axis_angle(&axis, angle);

        if axis == Vector3::<f32>::y_axis() {
            self.target_matrix = self.target_matrix * rotation.to_homogeneous();
        } else {
            self.target_matrix = rotation.to_homogeneous() * self.target_matrix;
        }
    }
    pub fn set_focal_length(&mut self, focal_length: f32, sensor_size: f32) {
        self.fov = 2.0 * f32::atan(sensor_size * 0.5 / focal_length);
    }
}
