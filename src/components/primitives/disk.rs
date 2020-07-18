use crate::graphics::{Mesh, SubMesh, Vertex};
use cgmath::Vector3;

pub struct Disk;
impl Disk {
    pub fn new(gl: &gl::Gl, radius: f32, resolution: u32) -> Mesh {
        let vertices = Disk::generate_vertices(radius, resolution, true);
        let indices = Disk::generate_indices(resolution);
        let submeshes = vec![SubMesh::new(gl, vertices, indices)];
        Mesh::new(submeshes)
    }

    pub fn generate_vertices(radius: f32, resolution: u32, flip: bool) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let y = if flip { -1.0 } else { 1.0 };
        let center_vertex = Vertex {
            position: Vector3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, y, 0.0),
        };

        let angle_offset = 2.0 * std::f32::consts::PI / resolution as f32;
        for i in 0..resolution {
            let angle = i as f32 * angle_offset;
            let position = Disk::get_point_on_circle(radius, angle);
            vertices.push(Vertex {
                position,
                normal: Vector3::new(0.0, y, 0.0),
            })
        }
        vertices.push(center_vertex);
        vertices
    }
    pub fn generate_indices(resolution: u32) -> Vec<u32> {
        let mut indices = Vec::new();
        for i in 0..resolution {
            let a = resolution;
            let b = if i == resolution - 1 { 0 } else { i + 1 };
            let c = i;
            indices.push(a);
            indices.push(b);
            indices.push(c);
        }
        indices
    }

    pub fn get_point_on_circle(radius: f32, angle: f32) -> Vector3<f32> {
        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        Vector3::new(x, 0.0, z)
    }
}
