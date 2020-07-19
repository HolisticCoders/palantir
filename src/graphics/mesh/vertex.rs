use cgmath::{Vector2, Vector3};

// TODO: Use trait to automatically generate layout
pub struct Vertex {
    pub position: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub uv: Vector2<f32>,
}
