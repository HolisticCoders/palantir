use crate::{
    IndexBuffer, ShaderProgram, Texture, Vertex, VertexArray, VertexBuffer, VertexBufferLayout,
};
use cgmath::prelude::*;
use cgmath::{Matrix4, Vector2, Vector3};
use std::cell::RefCell;
// use std::error::Error;
// use tobj::load_obj;

pub struct SubMesh {
    pub shader_index: Option<usize>,
    vertex_buffer: VertexBuffer,
    layout: VertexBufferLayout,
    index_buffer: IndexBuffer,
    vertex_array: VertexArray,
}
impl SubMesh {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, shader_index: Option<usize>) -> Self {
        let mut submesh = SubMesh {
            shader_index,
            vertex_buffer: VertexBuffer::new(vertices),
            layout: VertexBufferLayout::new(),
            index_buffer: IndexBuffer::new(indices),
            vertex_array: VertexArray::new(),
        };

        submesh.layout.push::<f32>(3); // Position
        submesh.layout.push::<f32>(3); // Normal
        submesh.layout.push::<f32>(2); // UV coordinates

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
    pub shaders: Vec<RefCell<ShaderProgram>>,
}

impl Mesh {
    pub fn new(submeshes: Vec<SubMesh>) -> Self {
        Mesh {
            submeshes,
            matrix: Matrix4::identity(),
            shaders: Vec::new(),
        }
    }
}
