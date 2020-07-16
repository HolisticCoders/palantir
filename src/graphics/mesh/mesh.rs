use crate::graphics::{IndexBuffer, Vertex, VertexArray, VertexBuffer, VertexBufferLayout};
use crate::resources::{self, Resources};
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector3};
use std::error::Error;
use tobj::load_obj;

pub enum MeshError {
    ResourceLoad {
        name: String,
        inner: resources::ResourceError,
    },
}
pub struct SubMesh {
    vertex_buffer: VertexBuffer,
    layout: VertexBufferLayout,
    index_buffer: IndexBuffer,
    vertex_array: VertexArray,
}
impl SubMesh {
    pub fn new(gl: &gl::Gl, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
        let mut submesh = SubMesh {
            vertex_buffer: VertexBuffer::new(gl, vertices),
            layout: VertexBufferLayout::new(),
            index_buffer: IndexBuffer::new(gl, indices),
            vertex_array: VertexArray::new(gl),
        };

        submesh.layout.push::<f32>(3);
        submesh.layout.push::<f32>(3);
        submesh
            .vertex_array
            .add_buffer(&submesh.vertex_buffer, &submesh.layout);
        submesh
    }
    pub fn index_buffer(&self) -> &IndexBuffer {
        &self.index_buffer
    }
    pub fn vertex_array(&self) -> &VertexArray {
        &self.vertex_array
    }
}

pub struct Mesh {
    pub submeshes: Vec<SubMesh>,
    pub matrix: Matrix4<f32>,
}

impl Mesh {
    pub fn new(submeshes: Vec<SubMesh>) -> Self {
        Mesh {
            submeshes,
            matrix: Matrix4::identity(),
        }
    }
    pub fn from_res(gl: &gl::Gl, res: &Resources, name: &str) -> Result<Self, Box<dyn Error>> {
        let path = res.resource_name_to_path(name);

        let (models, _) = load_obj(path, true)?;

        let mut submeshes = Vec::<SubMesh>::new();
        for model in models {
            let obj_mesh = &model.mesh; // TODO: implement support loading multiple objects from an obj
            let mut vertices: Vec<Vertex> = Vec::new();
            let indices = obj_mesh.indices.clone();
            for i in 0..obj_mesh.positions.len() / 3 {
                let position = Vector3::<f32>::new(
                    obj_mesh.positions[i * 3],
                    obj_mesh.positions[i * 3 + 1],
                    obj_mesh.positions[i * 3 + 2],
                );
                let normal = Vector3::<f32>::new(
                    obj_mesh.normals[i * 3],
                    obj_mesh.normals[i * 3 + 1],
                    obj_mesh.normals[i * 3 + 2],
                );
                vertices.push(Vertex { position, normal });
            }
            submeshes.push(SubMesh::new(&gl, vertices, indices));
        }
        Ok(Mesh::new(submeshes))
    }
}
