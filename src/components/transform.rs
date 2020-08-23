use cgmath::Matrix4;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TransformComponent {
    pub matrix: Matrix4<f32>,
}
