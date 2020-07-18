use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3};

pub struct Light {
    pub matrix: Matrix4<f32>,
    pub color: Vector3<f32>,
    pub ambient_strength: f32,
    pub power: f32,
}
impl Light {
    pub fn direction(&self) -> Vector3<f32> {
        self.matrix.transform_vector(Vector3::unit_z())
    }
}
