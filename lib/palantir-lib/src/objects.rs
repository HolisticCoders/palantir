use cgmath::{Matrix4, Vector3};

pub trait TCamera {
    fn matrix(&self) -> Matrix4<f32>;
    fn projection_matrix(&self) -> Matrix4<f32>;
}

pub trait TLight {
    fn set_color(&mut self, color: Vector3<f32>);
    fn set_ambient_strength(&mut self, ambient_strength: f32);
    fn set_power(&mut self, power: f32);

    fn matrix(&self) -> Matrix4<f32>;
    fn direction(&self) -> Vector3<f32>;
    fn color(&self) -> Vector3<f32>;
    fn ambient_strength(&self) -> f32;
    fn power(&self) -> f32;
}
