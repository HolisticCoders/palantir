use crate::graphics::{Mesh, SubMesh, Vertex};
use cgmath::Vector3;

pub struct Cube;
impl Cube {
    pub fn new(gl: &gl::Gl, size: f32) -> Mesh {
        #[rustfmt::skip]
        let vertices = vec![
            // Top
            Vertex { position: Vector3::new(size, size, size), normal: Vector3::new(0.0, 1.0, 0.0), },
            Vertex { position: Vector3::new(size, size, -size), normal: Vector3::new(0.0, 1.0, 0.0), },
            Vertex { position: Vector3::new(-size, size, -size), normal: Vector3::new(0.0, 1.0, 0.0), },
            Vertex { position: Vector3::new(-size, size, size), normal: Vector3::new(0.0, 1.0, 0.0), },
            // Bottom
            Vertex { position: Vector3::new(size, -size, size), normal: Vector3::new(0.0, -1.0, 0.0), },
            Vertex { position: Vector3::new(-size, -size, size), normal: Vector3::new(0.0, -1.0, 0.0), },
            Vertex { position: Vector3::new(-size, -size, -size), normal: Vector3::new(0.0, -1.0, 0.0), },
            Vertex { position: Vector3::new(size, -size, -size), normal: Vector3::new(0.0, -1.0, 0.0), },
            // Right
            Vertex { position: Vector3::new(size, size, size), normal: Vector3::new(1.0, 0.0, 0.0), },
            Vertex { position: Vector3::new(size, -size, size), normal: Vector3::new(1.0, 0.0, 0.0), },
            Vertex { position: Vector3::new(size, -size, -size), normal: Vector3::new(1.0, 0.0, 0.0), },
            Vertex { position: Vector3::new(size, size, -size), normal: Vector3::new(1.0, 0.0, 0.0), },
            // Left
            Vertex { position: Vector3::new(-size, size, size), normal: Vector3::new(-1.0, 0.0, 0.0), },
            Vertex { position: Vector3::new(-size, size, -size), normal: Vector3::new(-1.0, 0.0, 0.0), },
            Vertex { position: Vector3::new(-size, -size, -size), normal: Vector3::new(-1.0, 0.0, 0.0), },
            Vertex { position: Vector3::new(-size, -size, size), normal: Vector3::new(-1.0, 0.0, 0.0), },
            // Front
            Vertex { position: Vector3::new(size, size, size), normal: Vector3::new(0.0, 0.0, 1.0), },
            Vertex { position: Vector3::new(-size, size, size), normal: Vector3::new(0.0, 0.0, 1.0), },
            Vertex { position: Vector3::new(-size, -size, size), normal: Vector3::new(0.0, 0.0, 1.0), },
            Vertex { position: Vector3::new(size, -size, size), normal: Vector3::new(0.0, 0.0, 1.0), },
            // Back
            Vertex { position: Vector3::new(size, size, -size), normal: Vector3::new(0.0, 0.0, -1.0), },
            Vertex { position: Vector3::new(size, -size, -size), normal: Vector3::new(0.0, 0.0, -1.0), },
            Vertex { position: Vector3::new(-size, -size, -size), normal: Vector3::new(0.0, 0.0, -1.0), },
            Vertex { position: Vector3::new(-size, size, -size), normal: Vector3::new(0.0, 0.0, -1.0), },
        ];

        #[rustfmt::skip]
        let indices = vec![
            0, 1, 2, // top
            0, 2, 3,
            4, 5, 6, // bot
            4, 6, 7,
            8, 9, 10, // right
            8, 10, 11,
            12, 13, 14, // left
            12, 14, 15,
            16, 17, 18, // front
            16, 18, 19,
            20, 21, 22, // back
            20, 22, 23,
        ];
        let submesh = SubMesh::new(gl, vertices, indices);
        Mesh::new(vec![submesh])
    }
}
