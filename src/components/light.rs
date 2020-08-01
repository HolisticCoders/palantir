use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3};
use palantir_lib::TLight;

pub struct Light {
    matrix: Matrix4<f32>,
    color: Vector3<f32>,
    ambient_strength: f32,
    power: f32,
}
impl Light {
    pub fn new() -> Self {
        Light {
            matrix: Matrix4::identity(),
            color: Vector3::new(1.0, 1.0, 1.0),
            ambient_strength: 0.25,
            power: 1.0,
        }
    }

    pub fn set_matrix(&mut self, matrix: Matrix4<f32>) {
        self.matrix = matrix;
    }
}

impl TLight for Light {
    fn set_color(&mut self, color: Vector3<f32>) {
        self.color = color;
    }
    fn set_ambient_strength(&mut self, ambient_strength: f32) {
        self.ambient_strength = ambient_strength
    }
    fn set_power(&mut self, power: f32) {
        self.power = power
    }

    fn matrix(&self) -> Matrix4<f32> {
        self.matrix.clone()
    }
    fn direction(&self) -> Vector3<f32> {
        self.matrix.transform_vector(Vector3::unit_z())
    }
    fn color(&self) -> Vector3<f32> {
        self.color.clone()
    }
    fn ambient_strength(&self) -> f32 {
        self.ambient_strength
    }
    fn power(&self) -> f32 {
        self.power
    }
}
