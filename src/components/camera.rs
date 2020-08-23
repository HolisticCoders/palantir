use cgmath::prelude::*;
use cgmath::{Matrix4, PerspectiveFov, Rad, Vector3, Vector4};
use palantir_lib::TCamera;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    fov: f32,
    near_clip: f32,
    far_clip: f32,
    target_matrix: Matrix4<f32>,
    zoom_matrix: Matrix4<f32>,
    rotatey_matrix: Matrix4<f32>,
    rotatex_matrix: Matrix4<f32>,
    aspect: f32,
    distance: f32,
}

impl Camera {
    pub fn new(fov: f32, near_clip: f32, far_clip: f32, aspect: f32) -> Self {
        let mut camera = Camera {
            fov,
            near_clip,
            far_clip,
            aspect,
            target_matrix: Matrix4::<f32>::identity(),
            zoom_matrix: Matrix4::<f32>::identity(),
            rotatey_matrix: Matrix4::<f32>::identity(),
            rotatex_matrix: Matrix4::<f32>::identity(),
            distance: 1.0,
        };

        camera.zoom(-3.0);

        camera
    }
    pub fn from_focal_length(
        focal_length: f32,
        sensor_size: f32,
        near_clip: f32,
        far_clip: f32,
        aspect: f32,
    ) -> Self {
        let mut camera = Camera::new(0.0, near_clip, far_clip, aspect);
        camera.set_focal_length(focal_length, sensor_size);
        camera
    }
    pub fn pan(&mut self, x: f32, y: f32) {
        let mut vector = Vector3::new(x, -y, 0.0);
        vector *= self.distance * 0.1;
        let translation_matrix = Matrix4::from_translation(vector);
        let orientation_matrix: Matrix4<f32> = self.rotatex_matrix * self.rotatey_matrix;
        self.target_matrix = self.target_matrix
            * orientation_matrix.inverse_transform().unwrap()
            * translation_matrix
            * orientation_matrix;
    }
    pub fn zoom(&mut self, amount: f32) {
        let compensated_amount = amount * self.distance * 0.1;
        let translation = Vector3::new(0.0, 0.0, compensated_amount);
        self.zoom_matrix = self.zoom_matrix * Matrix4::from_translation(translation);
        self.distance = f32::abs(self.distance - compensated_amount).max(0.01);
    }
    pub fn rotate(&mut self, axis: Vector3<f32>, angle: f32) {
        let rotation = Matrix4::from_axis_angle(axis, Rad(angle));

        if axis == Vector3::<f32>::unit_y() {
            self.rotatey_matrix = self.rotatey_matrix * rotation;
        } else {
            self.rotatex_matrix = self.rotatex_matrix * rotation;
        }
    }
    pub fn focus(&mut self) {
        //TODO: refacto to focus on selection
        self.target_matrix.w = Vector4::new(0.0, 0.0, 0.0, 1.0);
    }
    pub fn set_focal_length(&mut self, focal_length: f32, sensor_size: f32) {
        self.fov = 2.0 * f32::atan(sensor_size * 0.5 / focal_length);
    }
    pub fn set_aspect_ratio(&mut self, aspect: f32) {
        self.aspect = aspect;
    }
}

impl TCamera for Camera {
    fn matrix(&self) -> Matrix4<f32> {
        self.zoom_matrix * self.rotatex_matrix * self.rotatey_matrix * self.target_matrix
    }

    fn projection_matrix(&self) -> Matrix4<f32> {
        PerspectiveFov {
            fovy: Rad(self.fov),
            aspect: self.aspect,
            near: self.near_clip,
            far: self.far_clip,
        }
        .to_perspective()
        .into()
    }
}
