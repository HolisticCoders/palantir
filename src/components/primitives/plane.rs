use cgmath::{Vector2, Vector3};
use palantir_lib::{Mesh, SubMesh, Vertex};

#[allow(dead_code)]
pub struct Plane;

#[allow(dead_code)]
impl Plane {
    pub fn new(size: f32) -> Mesh {
        let vertices = vec![
            Vertex {
                position: Vector3::new(size, 0.0, size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                uv: Vector2::new(1.0, 1.0),
            },
            Vertex {
                position: Vector3::new(size, 0.0, -size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                uv: Vector2::new(1.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-size, 0.0, -size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                uv: Vector2::new(0.0, 0.0),
            },
            Vertex {
                position: Vector3::new(-size, 0.0, size),
                normal: Vector3::new(0.0, 1.0, 0.0),
                uv: Vector2::new(0.0, 1.0),
            },
        ];
        let indices = vec![0, 1, 2, 0, 2, 3];

        let submesh = SubMesh::new(vertices, indices, None);
        Mesh::new(vec![submesh])
    }
}
