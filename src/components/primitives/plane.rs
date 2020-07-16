use crate::graphics::{Mesh, SubMesh, Vertex};
use cgmath::Vector3;

pub struct Plane;
impl Plane {
    pub fn new(gl: &gl::Gl, size: f32) -> Mesh {
        let vertices = vec![
            Vertex {
                position: Vector3::new(size, 0.0, size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                // uv: Vector2::new(1.0, 1.0),
            },
            Vertex {
                position: Vector3::new(-size, 0.0, size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                // uv: Vector2::new(-1.0, 1.0),
            },
            Vertex {
                position: Vector3::new(-size, 0.0, -size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                // uv: Vector2::new(-1.0, -1.0),
            },
            Vertex {
                position: Vector3::new(size, 0.0, -size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                // uv: Vector2::new(1.0, -1.0),
            },
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];

        let submesh = SubMesh::new(gl, vertices, indices);
        Mesh::new(vec![submesh])
    }
}
