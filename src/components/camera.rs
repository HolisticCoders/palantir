use cgmath::prelude::*;
use cgmath::{Matrix4, PerspectiveFov, Rad, Vector3, Vector4};

pub struct Camera {
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    local_matrix: Matrix4<f32>,
    target_matrix: Matrix4<f32>,
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
            local_matrix: Matrix4::<f32>::identity(),
            target_matrix: Matrix4::<f32>::identity(),
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
    pub fn view_matrix(&self) -> Matrix4<f32> {
        self.local_matrix * self.target_matrix
    }
    pub fn projection_matrix(&self) -> Matrix4<f32> {
        PerspectiveFov {
            fovy: Rad(self.fov),
            aspect: self.aspect,
            near: self.near_clip,
            far: self.far_clip,
        }
        .to_perspective()
        .into()
    }
    pub fn pan(&mut self, x: f32, y: f32) {
        let mut vector = Vector3::new(x, -y, 0.0);
        vector *= self.distance * 0.1;
        let transformation_matrix = Matrix4::from_translation(vector);
        self.target_matrix = transformation_matrix * self.target_matrix;
    }
    pub fn zoom(&mut self, amount: f32) {
        let compensated_amount = amount * self.distance * 0.1;
        let translation = Vector3::new(0.0, 0.0, compensated_amount);
        self.local_matrix = self.local_matrix * Matrix4::from_translation(translation);
        self.distance = f32::abs(self.distance - compensated_amount).max(0.01);
    }
    pub fn rotate(&mut self, axis: Vector3<f32>, angle: f32) {
        let rotation = Matrix4::from_axis_angle(axis, Rad(angle));

        if axis == Vector3::<f32>::unit_y() {
            self.target_matrix = self.target_matrix * rotation;
        } else {
            self.target_matrix = rotation * self.target_matrix;
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
